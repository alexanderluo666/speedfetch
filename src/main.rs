use std::fs;
use std::env;

fn kernel(){
    let kernel_info = fs::read_to_string("/proc/version").unwrap();
    for lines in kernel_info.lines(){
        println!("{}",lines);
    }

}

fn os(){
    let os_info = fs::read_to_string("/etc/os-release").unwrap();
    for lines in os_info.lines(){
        if lines.starts_with("PRETTY_NAME="){
            println!("{}",lines.replace("PRETTY_NAME=", ""));
        }
    }
}

fn shell(){
    let shell_info = env::var("SHELL");
    println!("{}", shell_info.expect("REASON").replace("/bin/", ""));
}

fn cpu(){
    let cpu_info = fs::read_to_string("/proc/cpuinfo").unwrap();
    println!("{}", cpu_info);
}

fn memory(){
    let memory_info = fs::read_to_string("/proc/meminfo").unwrap();
    println!("{}", memory_info);
}

fn main() {
    os();
    kernel();
    shell();
    cpu();
    memory();
}
