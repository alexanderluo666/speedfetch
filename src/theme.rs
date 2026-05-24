use crate::config::ThemeConfig;
use crate::distro_styles::{self, DistroStyle};

#[derive(Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

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

pub struct GradientAnimator {
    pub phase: f32,
    pub speed: f32,
    trail: [f32; 5],
}

impl GradientAnimator {
    pub fn new(speed: f32) -> Self {
        Self {
            phase: 0.0,
            speed,
            trail: [0.0; 5],
        }
    }

    pub fn step(&mut self) {
        for i in (1..5).rev() {
            self.trail[i] = self.trail[i - 1];
        }
        self.trail[0] = self.phase;

        self.phase += self.speed;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
    }

    pub fn sample_phases(&self) -> [f32; 5] {
        [
            self.phase,
            self.trail[0],
            self.trail[1],
            self.trail[2],
            self.trail[3],
        ]
    }
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

    pub fn gradient_speed(&self, distro: &str) -> f32 {
        distro_styles::distro_style(distro).speed
    }

    pub fn render_logo(
        &self,
        lines: &[String],
        distro: &str,
        animator: &GradientAnimator,
    ) -> Vec<String> {
        let style = distro_styles::distro_style(distro);
        let phases = animator.sample_phases();
        lines
            .iter()
            .enumerate()
            .map(|(i, line)| self.gradient_text(line, style, phases, i))
            .collect()
    }

    fn gradient_text(
        &self,
        text: &str,
        style: DistroStyle,
        phases: [f32; 5],
        line_index: usize,
    ) -> String {
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        if len == 0 {
            return String::new();
        }

        let len_f = len as f32;
        let line_shift = line_index as f32 * 0.06;
        let weights = [0.42_f32, 0.24, 0.16, 0.10, 0.08];
        let mut out = String::with_capacity(len * 18);

        for (i, ch) in chars.iter().enumerate() {
            let base = i as f32 / len_f + line_shift;
            let color = blur_sample(style, base, &phases, style.blur, &weights);
            write_ansi(&mut out, color);
            out.push(*ch);
        }

        out.push_str(&self.reset);
        out
    }

    pub fn label(&self, text: &str) -> String {
        colorize(&self.label_color, text, &self.reset)
    }

    pub fn value(&self, text: &str) -> String {
        colorize(&self.value_color, text, &self.reset)
    }
}

fn blur_sample(
    style: DistroStyle,
    base_t: f32,
    phases: &[f32; 5],
    blur: f32,
    weights: &[f32; 5],
) -> Rgb {
    let start = rgb_tuple(style.start);
    let mid = rgb_tuple(style.mid);
    let end = rgb_tuple(style.end);

    let mut acc_r = 0.0_f32;
    let mut acc_g = 0.0_f32;
    let mut acc_b = 0.0_f32;
    let mut w_sum = 0.0_f32;

    for (phase, &w) in phases.iter().zip(weights.iter()) {
        let mut pr = 0.0_f32;
        let mut pg = 0.0_f32;
        let mut pb = 0.0_f32;
        let mut phase_w = 0.0_f32;

        for k in 0..3 {
            let offset = (k as f32 - 1.0) * blur;
            let t = fract(base_t + phase + offset);
            let c = sample_gradient(start, mid, end, t);
            let kw = 1.0 - (k as f32 - 1.0).abs() * 0.35;
            pr += c.r as f32 * kw;
            pg += c.g as f32 * kw;
            pb += c.b as f32 * kw;
            phase_w += kw;
        }

        if phase_w > 0.0 {
            pr /= phase_w;
            pg /= phase_w;
            pb /= phase_w;
        }

        acc_r += pr * w;
        acc_g += pg * w;
        acc_b += pb * w;
        w_sum += w;
    }

    if w_sum > 0.0 {
        acc_r /= w_sum;
        acc_g /= w_sum;
        acc_b /= w_sum;
    }

    Rgb {
        r: acc_r.round() as u8,
        g: acc_g.round() as u8,
        b: acc_b.round() as u8,
    }
}

fn sample_gradient(start: Rgb, mid: Rgb, end: Rgb, t: f32) -> Rgb {
    let t = smoothstep(t);
    if t < 0.5 {
        mix(start, mid, t * 2.0)
    } else {
        mix(mid, end, (t - 0.5) * 2.0)
    }
}

fn rgb_tuple(t: (u8, u8, u8)) -> Rgb {
    Rgb {
        r: t.0,
        g: t.1,
        b: t.2,
    }
}

fn write_ansi(out: &mut String, color: Rgb) {
    use std::fmt::Write;
    let _ = write!(out, "\x1b[38;2;{};{};{}m", color.r, color.g, color.b);
}

fn smoothstep(t: f32) -> f32 {
    let t = fract(t);
    t * t * (3.0 - 2.0 * t)
}

fn mix(a: Rgb, b: Rgb, t: f32) -> Rgb {
    Rgb {
        r: lerp(a.r, b.r, t),
        g: lerp(a.g, b.g, t),
        b: lerp(a.b, b.b, t),
    }
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

fn fract(t: f32) -> f32 {
    t - t.floor()
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
