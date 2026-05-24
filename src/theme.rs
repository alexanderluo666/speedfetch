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
        if self.logo_color.is_empty() {
            return text.to_string();
        }

        format!(
            "{}{}{}",
            self.logo_color,
            text,
            self.reset
        )
    }

    pub fn label(&self, text: &str) -> String {
        if self.label_color.is_empty() {
            return text.to_string()
        }
        format!(
            "{}{}{}",
            self.label_color,
            text,
            self.reset
        )
    }

    pub fn value(&self, text: &str) -> String {
        if self.value_color.is_empty() {
            return text.to_string()
        }
        format!(
            "{}{}{}",
            self.value_color,
            text,
            self.reset
        )
    }
    pub fn fedora() -> Self {
        Self::new(
            "\x1b[34m".to_string(),
            "\x1b[38;5;208m".to_string(),
            "\x1b[37m".to_string(),
            "\x1b[0m".to_string(),
        )
    }

    pub fn ubuntu() -> Self {
        Self::new(
            "\x1b[38;5;208m".to_string(),
            "\x1b[35m".to_string(),
            "\x1b[37m".to_string(),
            "\x1b[0m".to_string(),
        )
    }

    pub fn debian() -> Self {
        Self::new(
            "\x1b[31m".to_string(),
            "\x1b[35m".to_string(),
            "\x1b[37m".to_string(),
            "\x1b[0m".to_string(),
        )
    }
    
    pub fn arch() -> Self {
        Self::new(
            "\x1b[96m".to_string(), // bright cyan logo
            "\x1b[90m".to_string(), // dim labels
            "\x1b[37m".to_string(), // soft white values
            "\x1b[0m".to_string(),
        )
    }
    
    pub fn unknown() -> Self {
        Self::new(
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "\x1b[0m".to_string(),
        )
    }
}