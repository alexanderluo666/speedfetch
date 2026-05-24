pub struct Theme {
    logo_color: String,
    label_color: String,
    value_color: String,
    reset: String,
}

impl Theme {
    pub fn new(
        logo_color: String,
        label_color: String,
        value_color: String,
        reset: String,
    ) -> Self {
        Self {
            logo_color,
            label_color,
            value_color,
            reset,
        }
    }

    pub fn logo(&self, text: &str) -> String {
        format!(
            "{}{}{}",
            self.logo_color,
            text,
            self.reset
        )
    }

    pub fn label(&self, text: &str) -> String {
        format!(
            "{}{}{}",
            self.label_color,
            text,
            self.reset
        )
    }

    pub fn value(&self, text: &str) -> String {
        format!(
            "{}{}{}",
            self.value_color,
            text,
            self.reset
        )
    }
    pub fn fedora() -> Self {
    Self::new(
        "\x1b[34m".to_string(),      // logo blue
        "\x1b[38;5;208m".to_string(), // orange labels
        "\x1b[37m".to_string(),      // white values
        "\x1b[0m".to_string(),       // reset
    )
}
}