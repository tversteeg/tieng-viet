use crate::grammar::{
    defaults_parser,
    phrase::{NounPhrase, VerbPhrase},
    word::Word,
    Generate,
};
use anyhow::{anyhow, Result};
use rand::Rng;

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
    ) -> Result<Box<dyn Iterator<Item = Word>>>
    where
        R: Rng,
    {
        Ok(Box::new(
            structure
                .into_iter()
                // Loop over all items in the structure and map them to the sub-structures
                .map(|item| match item.to_uppercase().as_str() {
                    "NOUN" => NounPhrase::generate(rng),
                    "VERB" => VerbPhrase::generate(rng),
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
        let allowed = Sentence::allowed_structures().collect::<Vec<_>>();
        assert!(allowed.len() > 0);

        Ok(())
    }
}
