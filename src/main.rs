use std::fs;
use std::env;
use std::cmp::max;
use unicode_width::UnicodeWidthStr;
use std::process::Command;

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

fn get_logo(){
    let distro_info = fs::read_to_string("/etc/os-release").unwrap();
    let mut distro = "";
    for line in distro_info.lines(){
        if line.starts_with("ID"){
            distro = &line.split("ID=").last().unwrap();
        }
    }
}

fn render(){
    let top_left: Vec<String> = vec![
        format!("Nothing yet..."),
        format!("Nothing yet..."),
        format!("Nothing yet...")
    ];
    let top_right: Vec<String> = vec![
        format!("OS: {}",os()),
        format!("Kernel: {}", kernel()),
        format!("Shell: {}",shell())
    ];
    let bottom_left: Vec<String> = vec![
        format!("Nothing yet..."),
        format!("Nothing yet..."),
        format!("Nothing yet..."),
        format!("Nothing yet...")
    ];
    let bottom_right: Vec<String> = vec![
        format!("CPU: {}", cpu()),
        format!("GPU: {}", gpu()),
        format!("Memory: {}", memory()),
        format!("Uptime: {} minutes", uptime())
    ];

    let top_left_width = top_left.iter().map(|s| s.width()).max().unwrap();
    let top_right_width = top_right.iter().map(|s| s.width()).max().unwrap();
    let bottom_left_width = bottom_left.iter().map(|s| s.width()).max().unwrap();
    let bottom_right_width = bottom_right.iter().map(|s| s.width()).max().unwrap();

    let top_height = max(top_left.len(),top_right.len());
    let bottom_height = max(bottom_left.len(),bottom_right.len());
    let left_width = max(top_left.iter().map(|s| s.width()).max().unwrap(),bottom_left.iter().map(|s| s.width()).max().unwrap());
    let right_width = max(top_right.iter().map(|s| s.width()).max().unwrap(),bottom_right.iter().map(|s| s.width()).max().unwrap());

    let gap = 4;
    let width = left_width + gap + right_width;

    println!("┌{}┐", "─".repeat(width as usize));
    
    for i in 0..top_height{
        let full_line = format!(
            "{:<left_width$}{}{}",
            top_left[i],
            " ".repeat(gap as usize),
            top_right[i]
        );
        println!("│{:width$}│", full_line);
    }

    println!("│{:width$}│", " ");

    for i in 0..bottom_height{
        let full_line = format!(
            "{:<left_width$}{}{}",
            bottom_left[i],
            " ".repeat(gap as usize),
            bottom_right[i]
        );
        println!("│{:width$}│", full_line);
    }

    println!("└{}┘", "─".repeat(width as usize));
}

fn main() {
    render();
}