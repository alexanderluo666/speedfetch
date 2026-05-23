use std::fs;
use std::env;
use std::cmp::max;
use unicode_width::UnicodeWidthStr;
use std::process::Command;

struct Panel {
    title: String,
    items: Vec<String>,
    width: usize
}

impl Panel {
    fn new(title: String, items: Vec<String>) -> Self {
        Self {
            title,
            items,
            width: 0,
        }
    }

    fn compute_width(&self) -> usize {
        let content_width = self
            .items
            .iter()
            .map(|s| s.width())
            .max()
            .unwrap_or(0);

        let title_width = self.title.width();

        std::cmp::max(content_width, title_width) + 4
    }

    fn render(&self) -> Vec<String> {
        let width = self.compute_width();
        let inner_width = width - 4;
        let mut lines = Vec::new();

        lines.push(format!("┌{}┐", "─".repeat(width - 2)));

        lines.push(format!(
            "│ {:<width$} │",
            self.title,
            width = inner_width
        ));

        lines.push(format!("├{}┤", "─".repeat(width - 2)));

        for item in &self.items {
            lines.push(format!(
                "│ {:<width$} │",
                item,
                width = inner_width
            ));
        }

        lines.push(format!("└{}┘", "─".repeat(width - 2)));

        lines
    }
}

fn os() -> String {
    for line in fs::read_to_string("/etc/os-release").unwrap().lines() {
        if line.starts_with("PRETTY_NAME=") {
            return line.replace("PRETTY_NAME=", "").replace("\"", "");
        }
    }
    return "Unknown OS".to_string();
}

fn kernel() -> String {
    let kernel_info = fs::read_to_string("/proc/sys/kernel/osrelease").unwrap().trim().to_string();
    format!("Linux {}", kernel_info)
}

fn shell() -> String {
    env::var("SHELL")
        .unwrap()
        .split("/")
        .last()
        .unwrap()
        .to_string()
}

fn cpu() -> String {
    let cpu_info = fs::read_to_string("/proc/cpuinfo").unwrap();
    for line in cpu_info.lines(){
        if line.starts_with("model name"){
            return line.split(": ").nth(1).expect("REASON").to_string();
        }
    }
    return "Unknown CPU".to_string();
}

fn memory() -> String {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap();

    let mut total = "";
    let mut available = "";
    let mut total_kb: f64;
    let mut available_kb: f64;

    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split(":").last().unwrap();
        }
        if line.starts_with("MemAvailable:") {
            available = line.split(":").last().unwrap();
        }
    }

    total = total.split("kB").next().unwrap().trim();
    total_kb = total.parse::<f64>().unwrap();
    available = available.split("kB").next().unwrap().trim();
    available_kb = available.parse::<f64>().unwrap();

    if total_kb > 1024.0*1024.0{
        total_kb = total_kb / 1024.0 / 1024.0;
        available_kb = available_kb / 1024.0 / 1024.0;
    } else {
        total_kb = total_kb / 1024.0;
        available_kb = available_kb / 1024.0;
    }

    total_kb = (total_kb * 10.0).round() / 10.0;
    available_kb = (available_kb * 10.0).round() / 10.0;

    let used_kb = ((total_kb - available_kb) * 10.0).round() / 10.0;
    let mut used_percentage = (used_kb / total_kb)*100.0;

    used_percentage = (used_percentage * 10.0).round() / 10.0;
    format!("{}GB / {}GB {}%",used_kb,total_kb,used_percentage)
}

fn uptime() -> String {
    let uptime_info = fs::read_to_string("/proc/uptime").unwrap().split(" ").next().unwrap().to_string();
    let mut uptime_minutes: f64 = uptime_info.parse().unwrap();
    uptime_minutes = (uptime_minutes / 60.0).round();
    return uptime_minutes.to_string()
}

fn gpu() -> String {
    let output = Command::new("lspci").output().unwrap();

    String::from_utf8(output.stdout)
        .unwrap_or_default()
        .lines()
        .find(|l| l.contains("VGA") || l.contains("3D") || l.contains("Display"))
        .unwrap_or("Unknown GPU")
        .split(':')
        .last()
        .unwrap_or("Unknown GPU")
        .trim()
        .to_string()
}

fn logo() -> Vec<String> {
    let distro_info = fs::read_to_string("/etc/os-release").unwrap();
    let mut distro = "unknown";

    for line in distro_info.lines() {
        if line.starts_with("ID=") {
            distro = line.trim_start_matches("ID=").trim_matches('"');
            break;
        }
    }

    if distro.contains("fedora") {
        return vec![
            "  _____        _                  ".to_string(),
            " |  ___|__  __| | ___  _ __ __ _  ".to_string(),
            " | |_ / _ \\/ _` |/ _ \\| '__/ _` | ".to_string(),
            " |  _|  __/ (_| | (_) | | | (_| | ".to_string(),
            " |_|  \\___|\\__,_|\\___/|_|  \\__,_| ".to_string(),
            "                                  ".to_string(),
        ];
    }

    // fallback
    vec![
        "  _____           _        ".to_string(),
        " |  __ \\         | |       ".to_string(),
        " | |  | | ___  __| | ___   ".to_string(),
        " | |  | |/ _ \\/ _` |/ _ \\  ".to_string(),
        " | |__| |  __/ (_| | (_) | ".to_string(),
        " |_____/ \\___|\\__,_|\\___/  ".to_string(),
    ]
}

fn compose() -> Vec<String> {
    let system_panel = Panel::new(
        "System".to_string(),
        vec![
            format!("OS: {}", os()),
            format!("Kernel: {}", kernel()),
            format!("Shell: {}", shell()),
        ],
    );

    let hardware_panel = Panel::new(
        "Hardware".to_string(),
        vec![
            format!("CPU: {}", cpu()),
            format!("GPU: {}", gpu()),
            format!("Memory: {}", memory()),
        ],
    );

    let session_panel = Panel::new(
        "Session".to_string(),
        vec![
            format!("Uptime: {} minutes", uptime()),
        ],
    );

    let logo_lines = logo();

    let system = system_panel.render();
    let hardware = hardware_panel.render();
    let session = session_panel.render();


    let mut left_column = Vec::new();
    left_column.extend(logo_lines);
    left_column.extend(session);

    let mut right_column = Vec::new();
    right_column.extend(system);
    right_column.extend(hardware);


    let max_height = left_column.len().max(right_column.len());

    while left_column.len() < max_height {
        left_column.push(String::new());
    }

    while right_column.len() < max_height {
        right_column.push(String::new());
    }

    let gap = 4;
    let mut output = Vec::new();

    for i in 0..max_height {
        let line = format!(
            "{:<30}{}{}",
            left_column[i],
            " ".repeat(gap),
            right_column[i]
        );

        output.push(line);
    }

    output
}

fn render() {
    let lines = compose();

    for line in lines {
        println!("{}", line);
    }
}

fn main() {
    render();
}