/// Parse a file consisting of lines.
pub fn parse_str(data: &str) -> impl Iterator<Item = &str> {
    data.lines()
        // Trim whitespace of lines
        .map(|line| line.trim())
        // Remove empty lines & comments
        .filter(|line| !line.is_empty() && !line.starts_with("#"))
}
