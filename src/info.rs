use std::env;
use std::fs;
use std::process::Command;

pub fn os() -> String {
    fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|data| {
            data.lines().find_map(|line| {
                line.strip_prefix("PRETTY_NAME=")
                    .map(|v| v.trim_matches('"').to_string())
            })
        })
        .unwrap_or_else(|| "Unknown OS".to_string())
}

pub fn kernel() -> String {
    fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|k| format!("Linux {}", k.trim()))
        .unwrap_or_else(|_| "Unknown".to_string())
}

pub fn hostname() -> String {
    env::var("HOSTNAME")
        .or_else(|_| env::var("HOST"))
        .unwrap_or_else(|_| {
            Command::new("hostname")
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "localhost".to_string())
        })
}

pub fn username() -> String {
    env::var("USER")
        .or_else(|_| env::var("LOGNAME"))
        .unwrap_or_else(|_| "user".to_string())
}

pub fn user_host() -> String {
    format!("{}@{}", username(), hostname())
}

pub fn shell() -> String {
    env::var("SHELL")
        .ok()
        .and_then(|s| s.rsplit('/').next().map(str::to_string))
        .unwrap_or_else(|| "sh".to_string())
}

pub fn terminal() -> String {
    if let Ok(t) = env::var("TERM_PROGRAM") {
        if let Ok(v) = env::var("TERM_PROGRAM_VERSION") {
            return format!("{t} {v}");
        }
        return t;
    }
    if let Ok(t) = env::var("TERMINAL_EMULATOR") {
        return t.replace("()", "").trim().to_string();
    }
    env::var("TERM").unwrap_or_else(|_| "unknown".to_string())
}

pub fn de_wm() -> String {
    if let Ok(de) = env::var("XDG_CURRENT_DESKTOP") {
        if !de.is_empty() {
            return de;
        }
    }
    if let Ok(s) = env::var("DESKTOP_SESSION") {
        if !s.is_empty() {
            return s;
        }
    }
    if let Ok(s) = env::var("XDG_SESSION_DESKTOP") {
        if !s.is_empty() {
            return s;
        }
    }
    if env::var("WAYLAND_DISPLAY").is_ok() {
        return "Wayland".to_string();
    }
    "N/A".to_string()
}

pub fn locale() -> String {
    env::var("LC_ALL")
        .or_else(|_| env::var("LANG"))
        .unwrap_or_else(|_| "C.UTF-8".to_string())
}

pub fn font() -> String {
    if let Ok(out) = Command::new("fc-match")
        .args(["-f", "%{family} %{size}pt", "monospace"])
        .output()
    {
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !s.is_empty() && s != "monospace" {
            return s;
        }
    }
    if let Ok(f) = env::var("KITTY_FONT_FAMILY") {
        return f;
    }
    if let Ok(f) = env::var("ALACRITTY_FONT_FAMILY") {
        return f;
    }
    "Default".to_string()
}

pub fn resolution() -> String {
    if let Ok(out) = Command::new("xrandr")
        .args(["--current"])
        .output()
    {
        for line in String::from_utf8_lossy(&out.stdout).lines() {
            if line.contains('*') {
                if let Some(res) = line.split_whitespace().nth(0) {
                    return res.to_string();
                }
            }
        }
    }
    if let Ok(out) = Command::new("wlr-randr").output() {
        for line in String::from_utf8_lossy(&out.stdout).lines() {
            if line.contains("current") && line.contains('x') {
                return line.split_whitespace().nth(1).unwrap_or("").to_string();
            }
        }
    }
    "N/A".to_string()
}

pub fn cpu() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|info| {
            info.lines().find_map(|line| {
                line.strip_prefix("model name\t: ")
                    .or_else(|| line.strip_prefix("model name:"))
                    .map(str::trim)
                    .map(str::to_string)
            })
        })
        .unwrap_or_else(|| "Unknown CPU".to_string())
}

pub fn gpu() -> String {
    Command::new("lspci")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|out| {
            out.lines()
                .find(|l| {
                    l.contains("VGA")
                        || l.contains("3D")
                        || l.contains("Display")
                        || l.contains("GPU")
                })
                .and_then(|l| l.split(':').nth(2).or_else(|| l.split(':').last()))
                .map(str::trim)
                .map(str::to_string)
        })
        .unwrap_or_else(|| "Unknown GPU".to_string())
}

pub fn memory() -> String {
    let meminfo = match fs::read_to_string("/proc/meminfo") {
        Ok(m) => m,
        Err(_) => return "Unknown".to_string(),
    };

    let mut total_kb = 0.0_f64;
    let mut avail_kb = 0.0_f64;

    for line in meminfo.lines() {
        if let Some(v) = line.strip_prefix("MemTotal:") {
            total_kb = parse_kb(v);
        } else if let Some(v) = line.strip_prefix("MemAvailable:") {
            avail_kb = parse_kb(v);
        }
    }

    if total_kb <= 0.0 {
        return "Unknown".to_string();
    }

    let (total, avail, unit) = if total_kb > 1024.0 * 1024.0 {
        (total_kb / 1024.0 / 1024.0, avail_kb / 1024.0 / 1024.0, "GB")
    } else {
        (total_kb / 1024.0, avail_kb / 1024.0, "MB")
    };

    let used = ((total - avail) * 10.0).round() / 10.0;
    let total = (total * 10.0).round() / 10.0;
    let pct = ((used / total) * 1000.0).round() / 10.0;
    format!("{used}{unit} / {total}{unit} ({pct}%)")
}

fn parse_kb(s: &str) -> f64 {
    s.split("kB")
        .next()
        .unwrap_or(s)
        .trim()
        .parse()
        .unwrap_or(0.0)
}

pub fn disk() -> String {
    Command::new("df")
        .args(["-h", "--output=used,size,pcent", "/"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|out| {
            out.lines()
                .nth(1)
                .map(|line| {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 {
                        format!("{} / {} ({})", parts[0], parts[1], parts[2])
                    } else {
                        line.trim().to_string()
                    }
                })
        })
        .unwrap_or_else(|| "N/A".to_string())
}

pub fn packages() -> String {
    if let Ok(out) = Command::new("rpm").args(["-qa"]).output() {
        if out.status.success() {
            let n = String::from_utf8_lossy(&out.stdout).lines().count();
            return format!("{n} (rpm)");
        }
    }
    if let Ok(out) = Command::new("dpkg").args(["--list"]).output() {
        if out.status.success() {
            let n = String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter(|l| l.starts_with("ii"))
                .count();
            return format!("{n} (dpkg)");
        }
    }
    if let Ok(out) = Command::new("pacman").args(["-Q"]).output() {
        if out.status.success() {
            let n = String::from_utf8_lossy(&out.stdout).lines().count();
            return format!("{n} (pacman)");
        }
    }
    if let Ok(out) = Command::new("apk").args(["info"]).output() {
        if out.status.success() {
            let n = String::from_utf8_lossy(&out.stdout).lines().count();
            return format!("{n} (apk)");
        }
    }
    if let Ok(out) = Command::new("nix-store").args(["-qR", "/run/current-system"]).output() {
        if out.status.success() {
            let n = String::from_utf8_lossy(&out.stdout).lines().count();
            return format!("{n} (nix)");
        }
    }
    "N/A".to_string()
}

pub fn uptime() -> String {
    let secs: f64 = fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| s.split_whitespace().next().map(str::to_string))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);

    let mins = (secs / 60.0).floor() as u64;
    let hours = mins / 60;
    let days = hours / 24;

    if days > 0 {
        format!("{days}d {}h", hours % 24)
    } else if hours > 0 {
        format!("{hours}h {}m", mins % 60)
    } else {
        format!("{mins}m")
    }
}

pub fn arch() -> String {
    if let Ok(a) = env::var("HOSTTYPE") {
        if !a.is_empty() {
            return a;
        }
    }
    if let Ok(info) = fs::read_to_string("/proc/cpuinfo") {
        if let Some(a) = info.lines().find_map(|l| {
            l.strip_prefix("CPU architecture: ")
                .or_else(|| l.strip_prefix("cpu family\t: "))
                .map(str::trim)
                .map(str::to_string)
        }) {
            return a;
        }
    }
    "unknown".to_string()
}

pub fn init_system() -> String {
    if fs::metadata("/run/systemd/system").is_ok() {
        return "systemd".to_string();
    }
    if fs::metadata("/sbin/openrc").is_ok() {
        return "OpenRC".to_string();
    }
    if fs::metadata("/sbin/runit").is_ok() {
        return "runit".to_string();
    }
    "N/A".to_string()
}
