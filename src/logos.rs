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


    vec![
        "  _____           _        ".to_string(),
        " |  __ \\         | |       ".to_string(),
        " | |  | | ___  __| | ___   ".to_string(),
        " | |  | |/ _ \\/ _` |/ _ \\  ".to_string(),
        " | |__| |  __/ (_| | (_) | ".to_string(),
        " |_____/ \\___|\\__,_|\\___/  ".to_string(),
    ]
}