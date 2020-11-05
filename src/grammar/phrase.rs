use crate::grammar::{defaults_parser, word::*, Generate};
use anyhow::{anyhow, Result};
use log::debug;
use rand::Rng;
use std::iter;

/// A phrase with a noun as it's head.
///
/// See: https://en.wikipedia.org/wiki/Noun_phrase
pub struct NounPhrase {}

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
        metadata: Vec<&str>,
    ) -> Result<Box<dyn Iterator<Item = Word>>>
    where
        R: Rng,
    {
        // If the structure contains a classifier use that as the base
        let classifier = if structure.iter().any(|item| item == "CLASSIFIER") {
            Some(Word::random_default(
                rng,
                Class::ClassifierNoun(ClassifierNoun::default()),
            )?)
        } else {
            None
        };

        debug!("NP: {:?}", metadata);
        Ok(Box::new(
            structure
                .into_iter()
                // Loop over all items in the structure and map them to the sub-structures
                .map(|item| match item.as_str() {
                    "DEMONSTRATIVE" => {
                        Word::random_default(rng, Class::Demonstrative(Demonstrative::default()))
                    }
                    // Can be safely unwrapped because it's defined above
                    "CLASSIFIER" => Ok(classifier.as_ref().unwrap().clone()),
                    "HEAD" => Word::random_default(
                        rng,
                        Class::ProperNoun(ProperNoun {
                            is_object: metadata.iter().any(|m| *m == "OBJECT"),
                            is_subject: metadata.iter().any(|m| *m == "SUBJECT"),
                            classifier: classifier.as_ref().map(|word| word.to_string()),
                        }),
                    ),
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
pub struct VerbPhrase {}

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
        metadata: Vec<&str>,
    ) -> Result<Box<dyn Iterator<Item = Word>>>
    where
        R: Rng,
    {
        debug!("VP: {:?}", metadata);
        Ok(Box::new(
            structure
                .into_iter()
                // Loop over all items in the structure and map them to the sub-structures
                .map(|item| match item.as_str() {
                    "VERB" => Ok(vec![Word::random_default(
                        rng,
                        Class::Verb(Verb::default()),
                    )?]),
                    "NOUN" => Ok(NounPhrase::generate(
                        rng,
                        // Put "OBJECT" in the metadata of the noun-phrase
                        metadata
                            .clone()
                            .into_iter()
                            .chain(iter::once("OBJECT"))
                            .collect(),
                    )?
                    .collect()),
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
