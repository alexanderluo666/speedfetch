use crate::config::ThemeConfig;

#[derive(Clone)]
pub struct Theme {
    pub logo_color: String,
    pub label_color: String,
    pub value_color: String,
    pub reset: String,
}

pub struct ThemeRegistry {
    themes: std::collections::HashMap<String, Theme>,
}

impl ThemeRegistry {
    pub fn from(config: &crate::config::Config) -> Self {
        let mut themes = std::collections::HashMap::new();

        for (name, distro) in &config.distro {
            themes.insert(name.clone(), Theme::from(&distro.theme));
        }

        Self { themes }
    }

    pub fn get(&self, distro: &str) -> Theme {
        self.themes
            .get(distro)
            .cloned()
            .or_else(|| self.themes.get("unknown").cloned())
            .unwrap_or_else(Theme::plain)
    }
}

impl Theme {
    pub fn from(cfg: &ThemeConfig) -> Self {
        Self {
            logo_color: map(&cfg.logo),
            label_color: map(&cfg.label),
            value_color: map(&cfg.value),
            reset: "\x1b[0m".to_string(),
        }
    }

    fn plain() -> Self {
        Self {
            logo_color: String::new(),
            label_color: String::new(),
            value_color: String::new(),
            reset: "\x1b[0m".to_string(),
        }
    }

    pub fn logo(&self, text: &str) -> String {
        colorize(&self.logo_color, text, &self.reset)
    }

    pub fn label(&self, text: &str) -> String {
        colorize(&self.label_color, text, &self.reset)
    }

    pub fn value(&self, text: &str) -> String {
        colorize(&self.value_color, text, &self.reset)
    }
}

fn colorize(color: &str, text: &str, reset: &str) -> String {
    if color.is_empty() {
        text.to_string()
    } else {
        format!("{color}{text}{reset}")
    }
}

fn map(s: &str) -> String {
    match s {
        "blue" => "\x1b[34m",
        "cyan" => "\x1b[96m",
        "orange" => "\x1b[38;5;208m",
        "purple" => "\x1b[35m",
        "red" => "\x1b[31m",
        "white" => "\x1b[37m",
        "dim" => "\x1b[90m",
        "none" => "",
        _ => "",
    }
    .to_string()
}
