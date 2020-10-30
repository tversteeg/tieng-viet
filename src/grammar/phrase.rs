use crate::grammar::{defaults_parser, word::*, Generate};
use anyhow::{anyhow, Result};
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

    fn default_words<'a, R>(
        rng: &'a mut R,
        structure: Self::StructureItem,
    ) -> Result<Box<dyn Iterator<Item = Word>>>
    where
        R: Rng,
    {
        Ok(Box::new(
            structure
                .into_iter()
                // Loop over all items in the structure and map them to the sub-structures
                .map(|item| match item.to_uppercase().as_str() {
                    "DEMONSTRATIVE" => {
                        Word::random_default(rng, Class::Demonstrative(Demonstrative::default()))
                    }
                    "CLASSIFIER" => {
                        Word::random_default(rng, Class::ClassifierNoun(ClassifierNoun::default()))
                    }
                    "HEAD" => Word::random_default(rng, Class::ProperNoun(ProperNoun::default())),
                    _ => Err(anyhow!("Unrecognized structure item {}", item)),
                })
                // Collect the vector so the random number generator is consumed.
                // TODO: bind the lifetime of the box to the lifetime of the RNG.
                .collect::<Result<Vec<_>>>()?
                .into_iter(),
        ))
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

    fn default_words<'a, R>(
        rng: &'a mut R,
        structure: Self::StructureItem,
    ) -> Result<Box<dyn Iterator<Item = Word>>>
    where
        R: Rng,
    {
        Ok(Box::new(
            structure
                .into_iter()
                // Loop over all items in the structure and map them to the sub-structures
                .map(|item| match item.to_uppercase().as_str() {
                    "VERB" => Ok(vec![Word::random_default(
                        rng,
                        Class::Verb(Verb::default()),
                    )?]),
                    "NOUN" => Ok(NounPhrase::generate(rng)?.collect()),
                    _ => Err(anyhow!("Unrecognized structure item {}", item)),
                })
                // Collect the vector so the random number generator is consumed.
                // TODO: bind the lifetime of the box to the lifetime of the RNG.
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .flatten(),
        ))
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
