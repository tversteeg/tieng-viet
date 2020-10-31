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

/// Parse a line with the format "word(metadata): description"
pub fn parse_word_line(line: &str) -> (&str, Vec<&str>, Option<&str>) {
    // Extract the description part :
    let (word_with_metadata, description) = line
        .split_once(":")
        .map(|(word_with_metadata, description)| (word_with_metadata, Some(description.trim())))
        .unwrap_or((line, None));

    let metadata_trim: &[_] = &[' ', ')'];

    // Extract the metadata part (..)
    let (word, metadata) = word_with_metadata
        .split_once("(")
        .map(|(word, metadata)| {
            (
                word,
                metadata
                    .trim_end_matches(metadata_trim)
                    .split("+")
                    .into_iter()
                    .map(|metadata| metadata.trim())
                    .collect(),
            )
        })
        .unwrap_or((word_with_metadata, vec![]));

    (word.trim(), metadata, description)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_lines() {
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
    }

    #[test]
    fn test_word_line() {
        let (word, metadata, description) = parse_word_line("word");
        assert_eq!(word, "word");
        assert!(metadata.is_empty());
        assert!(description.is_none());

        let (word, metadata, description) = parse_word_line("word (METADATA)");
        assert_eq!(word, "word");
        assert_eq!(metadata, vec!["METADATA"]);
        assert!(description.is_none());

        let (word, metadata, description) = parse_word_line("word (METADATA + METADATA2)");
        assert_eq!(word, "word");
        assert_eq!(metadata, vec!["METADATA", "METADATA2"]);
        assert!(description.is_none());

        let (word, metadata, description) = parse_word_line("word (METADATA) : some description");
        assert_eq!(word, "word");
        assert_eq!(metadata, vec!["METADATA"]);
        assert_eq!(description, Some("some description"));
    }
}
