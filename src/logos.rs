use std::fs;

pub fn logo() -> Vec<String> {
    let distro_info = fs::read_to_string("/etc/os-release").unwrap();
    let mut distro = "unknown";

    for line in distro_info.lines() {
        if line.starts_with("ID=") {
            distro = line.trim_start_matches("ID=").trim_matches('"');
            break;
        }
    }
    
    let fedora: Vec<String> = vec![
        "\x1b[34m  _____        _                  ".to_string(),
        " |  ___|__  __| | ___  _ __ __ _  ".to_string(),
        " | |_ / _ \\/ _` |/ _ \\| '__/ _` | ".to_string(),
        " |  _|  __/ (_| | (_) | | | (_| | ".to_string(),
        " |_|  \\___|\\__,_|\\___/|_|  \\__,_| ".to_string(),
        "                                 ".to_string(),
    ];

    let ubuntu: Vec<String> = vec![
        "\x1b[38;5;166m  _   _ _                 _         ".to_string(),
        " | | | | |__  _   _ _ __ | |_ _   _ ".to_string(),
        " | | | | '_ \\| | | | '_ \\| __| | | |".to_string(),
        " | |_| | |_) | |_| | | | | |_| |_| |".to_string(),
        "  \\___/|_.__/ \\__,_|_| |_|\\__|\\__,_|".to_string(),
        "                                    ".to_string(),
    ];

    let unknown: Vec<String> = vec![
        "  _   _       _                              ".to_string(),
        " | | | |_ __ | | ___ __   _____      ___ __  ".to_string(),
        " | | | | '_ \\| |/ / '_ \\ / _ \\ \\ /\\ / / '_ \\ ".to_string(),
        " | |_| | | | |   <| | | | (_) \\ V  V /| | | |".to_string(),
        "  \\___/|_| |_|_|\\_\\_| |_|\\___/ \\_/\\_/ |_| |_|".to_string(),
        "                                             ".to_string(),
    ];
    match distro{
        "fedora" => return fedora,
        "ubuntu" => return ubuntu,
        _ => return unknown,
    }
}