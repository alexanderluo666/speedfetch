use std::fs;
use std::env;

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
    let info: Vec<String> = vec![
        format!("OS: {}",os()),
        format!("Kernel: {}",kernel()),
        format!("Shell: {}",shell()),
        format!("CPU: {}",cpu()),
        format!("Memory: {}",memory())
    ];
    let width = info.iter().map(|s| s.len()).max().unwrap();
    println!("┌{}┐", "─".repeat(width + 2));
    println!("│ {:<width$} │", format!("OS: {}", os()));
    println!("│ {:<width$} │", format!("Kernel: {}", kernel()));
    println!("│ {:<width$} │", format!("Shell: {}", shell()));
    println!("│ {:<width$} │", format!("CPU: {}", cpu()));
    println!("│ {:<width$} │", format!("Memory: {}", memory()));
    println!("└{}┘", "─".repeat(width + 2));
}

fn main() {
    render();
}