mod defaults_parser;
pub mod phrase;
pub mod sentence;
pub mod word;

use crate::grammar::word::Word;
use anyhow::{anyhow, Result};
use rand::{seq::IteratorRandom, Rng};

/// Add functionality that generates random phrases.
pub trait Generate {
    /// The type for the iterator.
    type StructureItem;

    /// Get a list of allowed structures from a txt source file.
    fn allowed_structures() -> Box<dyn Iterator<Item = Self::StructureItem>>;

    /// Get a list of words generated from a selected structure.
    fn words_from_structure<R>(
        &self,
        structure: Self::StructureItem,
        rng: &mut R,
    ) -> Box<dyn Iterator<Item = Word>>
    where
        R: Rng;

    /// Generate a sentence from the allowed structures.
    fn generate<R>(rng: &mut R) -> Result<Vec<Word>>
    where
        R: Rng,
    {
        // Select a random structure.
        Self::allowed_structures()
            .choose(rng)
            .ok_or(anyhow!("Could not get random structure for sentence"))?;

        todo!()
    }
}
