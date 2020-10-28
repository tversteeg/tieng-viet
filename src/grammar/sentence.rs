use crate::grammar::{
    defaults_parser,
    phrase::{NounPhrase, VerbPhrase},
    word::Word,
    Generate,
};
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

    fn words_from_structure<R>(
        &self,
        structure: Self::StructureItem,
        rng: &mut R,
    ) -> Box<dyn Iterator<Item = Word>>
    where
        R: Rng,
    {
        Box::new(
            structure
                .into_iter()
                .map(|item| match item.to_uppercase().as_str() {
                    "NOUN" => NounPhrase::generate(rng)
                        .expect("Could not generate nounphrase for sentence")
                        .into_iter(),
                    _ => unimplemented!(),
                })
                .flatten(),
        )
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
