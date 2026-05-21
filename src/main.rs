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
    fs::read_to_string("/proc/version")
        .unwrap()
        .split("x86_64")
        .next()
        .unwrap()
        .trim()
        .to_string()
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

    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total = line;
        }
        if line.starts_with("MemAvailable:") {
            available = line;
        }
    }
    format!("{}\n{}", total, available)
}

fn main() {
    println!("{}",os());
    println!("{}",kernel());
    println!("{}",shell());
    println!("{}",cpu());
    println!("{}",memory());
}
