pub fn distro() -> String {
    std::fs::read_to_string("/etc/os-release")
        .unwrap_or_default()
        .lines()
        .find_map(|line| {
            line.strip_prefix("ID=")
                .map(|v| v.trim_matches('"').to_string())
        })
        .unwrap_or_else(|| "unknown".to_string())
}