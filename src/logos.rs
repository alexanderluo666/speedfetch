use std::fs;

pub struct Branding {
    pub logo: Vec<String>,
    pub theme: crate::theme::Theme,
}

fn distro() -> String {
    if let Ok(fake) = std::env::var("DISTRO") {
        return fake;
    }

    let data = fs::read_to_string("/etc/os-release").unwrap();

    let mut id = None;

    for line in data.lines() {
        let line = line.trim();

        if let Some(v) = line.strip_prefix("ID=") {
            id = Some(v.trim_matches('"').to_string());
        }
    }

    id.unwrap_or_else(|| "unknown".to_string())
}

pub fn branding() -> Branding {
    let distro = distro();

    let fedora_logo: Vec<String> = vec![
        "  _____        _                  ".to_string(),
        " |  ___|__  __| | ___  _ __ __ _  ".to_string(),
        " | |_ / _ \\/ _` |/ _ \\| '__/ _` | ".to_string(),
        " |  _|  __/ (_| | (_) | | | (_| | ".to_string(),
        " |_|  \\___|\\__,_|\\___/|_|  \\__,_| ".to_string(),
        "                                  ".to_string(),
    ];

    let ubuntu_logo: Vec<String> = vec![
        "  _   _ _                 _         ".to_string(),
        " | | | | |__  _   _ _ __ | |_ _   _ ".to_string(),
        " | | | | '_ \\| | | | '_ \\| __| | | |".to_string(),
        " | |_| | |_) | |_| | | | | |_| |_| |".to_string(),
        "  \\___/|_.__/ \\__,_|_| |_|\\__|\\__,_|".to_string(),
        "                                    ".to_string(),
    ];

    let debian_logo: Vec<String> = vec![
        "  ____       _     _             ".to_string(),
        " |  _ \\  ___| |__ (_) __ _ _ __  ".to_string(),
        " | | | |/ _ \\ '_ \\| |/ _` | '_ \\ ".to_string(),
        " | |_| |  __/ |_) | | (_| | | | |".to_string(),
        " |____/ \\___|_.__/|_|\\__,_|_| |_|".to_string(),
        "                                 ".to_string(),
    ];

    let arch_logo: Vec<String> = vec![
        "     _             _     ".to_string(),
        "    / \\   _ __ ___| |__  ".to_string(),
        "   / _ \\ | '__/ __| '_ \\ ".to_string(),
        "  / ___ \\| | | (__| | | |".to_string(),
        " /_/   \\_\\_|  \\___|_| |_|".to_string(),
        "                         ".to_string(),
    ];

    let unknown_logo: Vec<String> = vec![
        "  _   _       _                              ".to_string(),
        " | | | |_ __ | | ___ __   _____      ___ __  ".to_string(),
        " | | | | '_ \\| |/ / '_ \\ / _ \\ \\ /\\ / / '_ \\ ".to_string(),
        " | |_| | | | |   <| | | | (_) \\ V  V /| | | |".to_string(),
        "  \\___/|_| |_|_|\\_\\_| |_|\\___/ \\_/\\_/ |_| |_|".to_string(),
        "                                             ".to_string(),
    ];
    println!("DISTRO DETECTED: '{}'", distro);
    match distro.as_str() {
        "fedora" => Branding {
            logo: fedora_logo,
            theme: crate::theme::Theme::fedora(),
        },
        
        "ubuntu" => Branding {
            logo: ubuntu_logo,
            theme: crate::theme::Theme::ubuntu(),
        },

        "debian" => Branding {
            logo: debian_logo,
            theme: crate::theme::Theme::debian(),
        },
        "arch" => Branding {
            logo: arch_logo,
            theme: crate::theme::Theme::arch()
        },
        _ => Branding {
            logo: unknown_logo,
            theme: crate::theme::Theme::unknown(),
        },
    }
}