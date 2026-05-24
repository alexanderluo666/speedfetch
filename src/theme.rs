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

    /// Default logo gradient color names per distro (start → end).
    pub fn logo_gradient_stops(distro: &str) -> Option<(&'static str, &'static str)> {
        match distro {
            "fedora" => Some(("blue", "cyan")),
            "ubuntu" => Some(("orange", "red")),
            "debian" => Some(("red", "magenta")),
            "arch" => Some(("cyan", "blue")),
            "unknown" => Some(("gray", "white")),
            _ => None,
        }
    }

    pub fn gradient(&self, text: &str, start_color: &str, end_color: &str) -> String {
        if start_color.is_empty()
            || end_color.is_empty()
            || start_color == "none"
            || end_color == "none"
        {
            return self.logo(text);
        }

        let Some((sr, sg, sb)) = color_rgb(start_color) else {
            return self.logo(text);
        };
        let Some((er, eg, eb)) = color_rgb(end_color) else {
            return self.logo(text);
        };

        let chars: Vec<char> = text.chars().collect();
        if chars.is_empty() {
            return String::new();
        }

        let last = chars.len().saturating_sub(1);
        let mut out = String::with_capacity(text.len() * 12);

        for (i, ch) in chars.iter().enumerate() {
            let t = if last == 0 { 0.0 } else { i as f32 / last as f32 };
            let r = lerp_u8(sr, er, t);
            let g = lerp_u8(sg, eg, t);
            let b = lerp_u8(sb, eb, t);
            out.push_str(&format!("\x1b[38;2;{r};{g};{b}m{ch}"));
        }

        out.push_str(&self.reset);
        out
    }

    pub fn logo_gradient(&self, lines: &[String], start: &str, end: &str) -> Vec<String> {
        if start.is_empty()
            || end.is_empty()
            || start == "none"
            || end == "none"
        {
            return lines.iter().map(|l| self.logo(l)).collect();
        }

        lines
            .iter()
            .map(|line| self.gradient(line, start, end))
            .collect()
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
        "magenta" => "\x1b[35m",
        "white" => "\x1b[37m",
        "gray" => "\x1b[90m",
        "dim" => "\x1b[90m",
        "none" => "",
        _ => "",
    }
    .to_string()
}

fn color_rgb(name: &str) -> Option<(u8, u8, u8)> {
    Some(match name {
        "blue" => (0, 102, 255),
        "cyan" => (0, 255, 255),
        "orange" => (255, 135, 0),
        "red" => (255, 0, 0),
        "magenta" => (255, 0, 255),
        "purple" => (180, 0, 255),
        "white" => (255, 255, 255),
        "gray" => (128, 128, 128),
        "dim" => (128, 128, 128),
        _ => return None,
    })
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}
