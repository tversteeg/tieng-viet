use crate::grammar::{defaults_parser, word::*};

/// A phrase with a noun as it's head.
///
/// See: https://en.wikipedia.org/wiki/Noun_phrase
pub struct NounPhrase {
    /// The list of words in order that make up the noun-phrase.
    words: Vec<Word>,
}

impl NounPhrase {
    /// Combinations of classes that are allowed as a noun phrase.
    pub fn allowed_structures() -> impl Iterator<Item = Vec<String>> {
        defaults_parser::parse_str(include_str!("../noun_phrases.txt")).map(|line| {
            line.split("+")
                .map(|r#type| r#type.trim().to_string())
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_allowed() -> Result<()> {
        let allowed = NounPhrase::allowed_structures().collect::<Vec<_>>();

        assert!(allowed.len() > 0);

        Ok(())
    }
}
