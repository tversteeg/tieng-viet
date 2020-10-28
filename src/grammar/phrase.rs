use crate::grammar::{defaults_parser, word::Word, Generate};
use rand::Rng;

/// A phrase with a noun as it's head.
///
/// See: https://en.wikipedia.org/wiki/Noun_phrase
pub struct NounPhrase {
    /// The list of words in order that make up the noun-phrase.
    words: Vec<Word>,
}

impl Generate for NounPhrase {
    type StructureItem = Vec<String>;

    /// Combinations of classes that are allowed as a noun phrase.
    fn allowed_structures() -> Box<dyn Iterator<Item = Vec<String>>> {
        Box::new(
            defaults_parser::parse_str(include_str!("../noun_phrases.txt")).map(|line| {
                line.split("+")
                    .map(|r#type| r#type.trim().to_string())
                    .collect()
            }),
        )
    }

    fn words_from_structure<R>(
        &self,
        structure: Self::StructureItem,
        rng: &mut R,
    ) -> Box<dyn Iterator<Item = Word>>
    where
        R: Rng,
    {
        todo!()
    }
}

/// A phrase with a verb as it's head.
pub struct VerbPhrase {
    /// The list of words in order that make up the verb-phrase.
    words: Vec<Word>,
}

impl Generate for VerbPhrase {
    type StructureItem = Vec<String>;

    /// Combinations of classes that are allowed as a verb phrase.
    fn allowed_structures() -> Box<dyn Iterator<Item = Vec<String>>> {
        Box::new(
            defaults_parser::parse_str(include_str!("../verb_phrases.txt")).map(|line| {
                line.split("+")
                    .map(|r#type| r#type.trim().to_string())
                    .collect()
            }),
        )
    }

    fn words_from_structure<R>(
        &self,
        structure: Self::StructureItem,
        rng: &mut R,
    ) -> Box<dyn Iterator<Item = Word>>
    where
        R: Rng,
    {
        todo!()
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

        let allowed = VerbPhrase::allowed_structures().collect::<Vec<_>>();
        assert!(allowed.len() > 0);

        Ok(())
    }
}
