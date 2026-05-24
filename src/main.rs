use std::thread;
use std::time::Duration;

use unicode_width::UnicodeWidthStr;

mod config;
mod distro;
mod distro_styles;
mod info;
mod loader;
mod theme;
mod utils;

use config::{Config, DistroConfig};

struct Panel {
    title: String,
    items: Vec<String>,
}

impl Panel {
    fn new(title: String, items: Vec<String>) -> Self {
        Self { title, items }
    }

    fn compute_width(&self) -> usize {
        let content_width = self
            .items
            .iter()
            .map(|s| utils::strip_ansi(s).width())
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
            "│ {:<inner_width$} │",
            self.title,
            inner_width = inner_width
        ));

        lines.push(format!("├{}┤", "─".repeat(width - 2)));

        for item in &self.items {
            let visible = utils::strip_ansi(item).width();
            let padding = inner_width.saturating_sub(visible);

            // IMPORTANT: build final padded string BEFORE wrapping border
            let mut line = String::new();
            line.push_str(item);
            line.push_str(&" ".repeat(padding));

            lines.push(format!("│ {} │", line));
        }

        lines.push(format!("└{}┘", "─".repeat(width - 2)));

        lines
    }
}

fn get_distro() -> String {
    let mut args = std::env::args();

    while let Some(arg) = args.next() {
        if arg == "--distro" {
            return args.next().unwrap_or_else(|| "unknown".to_string());
        }

        if let Some(v) = arg.strip_prefix("--distro=") {
            return v.to_string();
        }
    }

    distro::distro()
}

fn resolve_inheritance(config: &Config, entry: DistroConfig) -> DistroConfig {
    if entry.inherits.is_empty() {
        return entry;
    }

    let child_theme = entry.theme;
    let child_logo = entry.logo;
    let parent_key = entry.inherits.clone();

    let mut resolved = config
        .distro
        .get(&parent_key)
        .map(|p| resolve_inheritance(config, p.clone()))
        .unwrap_or_else(|| DistroConfig {
            inherits: String::new(),
            logo: child_logo.clone(),
            theme: child_theme.clone(),
        });

    if !child_logo.is_empty() {
        resolved.logo = child_logo;
    }
    resolved.theme = child_theme;
    resolved.inherits = String::new();
    resolved
}

fn compose(config: &Config, distro: &str, animator: &theme::GradientAnimator) -> Vec<String> {
    let logo_key = if config.distro.contains_key(distro) {
        distro.to_string()
    } else {
        distro_styles::logo_family(distro).to_string()
    };

    let entry = config
        .distro
        .get(&logo_key)
        .or_else(|| config.distro.get("unknown"))
        .expect("unknown distro must exist in config")
        .clone();

    let entry = resolve_inheritance(config, entry);

    let registry = theme::ThemeRegistry::from(config);
    let theme_key = if config.distro.contains_key(distro) {
        distro
    } else {
        distro_styles::logo_family(distro)
    };
    let theme = registry.get(theme_key);

    let logo_lines = theme.render_logo(&entry.logo, distro, animator);

    let row = |label: &str, value: &str| {
        format!("{} {}", theme.label(label), theme.value(value))
    };

    let system_panel = Panel::new(
        "System".into(),
        vec![
            row("OS:", &info::os()),
            row("Host:", &info::hostname()),
            row("Kernel:", &info::kernel()),
            row("Arch:", &info::arch()),
            row("Init:", &info::init_system()),
            row("Pkgs:", &info::packages()),
        ],
    );

    let session_panel = Panel::new(
        "Session".into(),
        vec![
            row("User:", &info::user_host()),
            row("Shell:", &info::shell()),
            row("Term:", &info::terminal()),
            row("DE/WM:", &info::de_wm()),
            row("Uptime:", &info::uptime()),
            row("Locale:", &info::locale()),
        ],
    );

    let hardware_panel = Panel::new(
        "Hardware".into(),
        vec![
            row("CPU:", &info::cpu()),
            row("GPU:", &info::gpu()),
            row("Memory:", &info::memory()),
            row("Disk:", &info::disk()),
        ],
    );

    let display_panel = Panel::new(
        "Display".into(),
        vec![
            row("Res:", &info::resolution()),
            row("Font:", &info::font()),
        ],
    );

    let system = system_panel.render();
    let session = session_panel.render();
    let hardware = hardware_panel.render();
    let display = display_panel.render();

    let mut left = Vec::new();
    left.extend(logo_lines);
    left.extend(session);

    let mut right = Vec::new();
    right.extend(system);
    right.extend(hardware);
    right.extend(display);

    let height = std::cmp::max(left.len(), right.len());

    let left_width = left
        .iter()
        .map(|l| utils::strip_ansi(l).width())
        .max()
        .unwrap_or(0);

    left.resize(height, String::new());
    right.resize(height, String::new());

    let mut output = Vec::new();

    for i in 0..height {
        let visible = utils::strip_ansi(&left[i]).width();
        let padding = left_width.saturating_sub(visible);

        let left_line = format!("{}{}", left[i], " ".repeat(padding));

        output.push(format!(
            "{}    {}",
            left_line,
            right[i]
        ));
    }

    output
}

fn print_frame(lines: &[String]) {
    print!("\x1b[2J\x1b[H");
    for line in lines {
        println!("{}", line);
    }
}

fn main() {
    let distro = get_distro();
    let config = loader::load_config();
    let speed = theme::ThemeRegistry::from(&config)
        .get(&distro)
        .gradient_speed(&distro);

    let mut animator = theme::GradientAnimator::new(speed);

    loop {
        animator.step();
        print_frame(&compose(&config, &distro, &animator));
        thread::sleep(Duration::from_millis(33));
    }
}