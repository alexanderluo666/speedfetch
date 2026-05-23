pub fn strip_ansi(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // skip ESC sequence
            while let Some(&next) = chars.peek() {
                chars.next();

                if next == 'm' {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}
