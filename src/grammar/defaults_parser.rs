/// Parse a file consisting of lines.
pub fn parse_str(data: &str) -> impl Iterator<Item = &str> {
    data.lines()
        // Remove comments at the end of the line
        .map(|line| line.split_once("#").map(|(data, _)| data).unwrap_or(line))
        // Trim whitespace of lines
        .map(|line| line.trim())
        // Remove empty lines & comments
        .filter(|line| !line.is_empty() && !line.starts_with("#"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use indoc::indoc;

    #[test]
    fn test_parse() -> Result<()> {
        let mut iter = parse_str(indoc!(
            r#"
            # A comment
            A line with data
            Data + comment # This part should be ignored
            "#
        ));
        assert_eq!(iter.next(), Some("A line with data"));
        assert_eq!(iter.next(), Some("Data + comment"));
        assert_eq!(iter.next(), None);

        Ok(())
    }
}
