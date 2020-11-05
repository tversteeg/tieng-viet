use crate::grammar::{
    defaults_parser,
    phrase::{NounPhrase, VerbPhrase},
    word::Word,
    Generate,
};
use anyhow::{anyhow, Result};
use log::debug;
use rand::Rng;
use std::iter;

/// A whole grammatical sentent.
#[derive(Debug, Clone, Default, PartialEq, Hash)]
pub struct Sentence {}

impl Generate for Sentence {
    type StructureItem = Vec<String>;

    fn allowed_structures() -> Box<dyn Iterator<Item = Vec<String>>> {
        Box::new(
            defaults_parser::parse_str(include_str!("../sentences.txt")).map(|line| {
                line.split("+")
                    .map(|r#type| r#type.trim().to_string())
                    .collect()
            }),
        )
    }

    fn default_words<'a, R>(
        rng: &'a mut R,
        structure: Vec<String>,
        metadata: Vec<&str>,
    ) -> Result<Box<dyn Iterator<Item = Word>>>
    where
        R: Rng,
    {
        debug!("S: {:?}", metadata);
        Ok(Box::new(
            structure
                .into_iter()
                // Loop over all items in the structure and map them to the sub-structures
                .map(|item| match item.to_uppercase().as_str() {
                    // Match both subject & object noun phrases
                    "SUBJECT" | "OBJECT" => NounPhrase::generate(
                        rng,
                        // Put "SUBJECT" or "OBJECT" in the metadata
                        metadata
                            .clone()
                            .into_iter()
                            .chain(iter::once(item.to_uppercase().as_str()))
                            .collect(),
                    ),
                    "VP" => VerbPhrase::generate(rng, metadata.clone()),
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

    fn init() {
        // Initialize the logger so we can see the logs when unit tests fail
        // Can be used with RUST_LOG=tieng_viet=trace cargo test
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_allowed() -> Result<()> {
        init();

        let allowed = Sentence::allowed_structures().collect::<Vec<_>>();
        assert!(allowed.len() > 0);

        Ok(())
    }

    #[test]
    fn test_generate() -> Result<()> {
        init();

        let mut rng = rand::thread_rng();

        let words = Sentence::generate(&mut rng, vec![])?.collect::<Vec<_>>();
        assert!(!words.is_empty());

        Ok(())
    }
}
