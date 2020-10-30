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

    /// Get a list of default words generated from a chosen structure.
    fn default_words<'a, R>(
        rng: &'a mut R,
        structure: Self::StructureItem,
    ) -> Result<Box<dyn Iterator<Item = Word>>>
    where
        R: Rng;

    /// Generate a sentence from the allowed structures.
    fn generate<'a, R>(rng: &'a mut R) -> Result<Box<dyn Iterator<Item = Word>>>
    where
        R: Rng,
    {
        // Select a random structure.
        let structure = Self::allowed_structures()
            .choose(rng)
            .ok_or(anyhow!("Could not get random structure for sentence"))?;

        // Get the default words from the structure.
        Self::default_words(rng, structure)
    }
}
