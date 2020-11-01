use crate::grammar::defaults_parser;
use anyhow::{anyhow, Result};
use log::debug;
use rand::{seq::IteratorRandom, Rng};
use std::fmt::Display;

/// A single word.
///
/// **Đi**: Go.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Word {
    /// The actual word as a string.
    content: String,
    /// Rough translation of the word in English.
    meaning: String,
    /// How this word is classified.
    class: Class,
}

impl Word {
    /// Parse the included text files and generate a list of words from that.
    pub fn defaults() -> impl Iterator<Item = Word> {
        // Parse the classifiers and add them
        defaults_parser::parse_str(include_str!("../classifiers.txt"))
            .map(|line| {
                let (word, _, meaning) = defaults_parser::parse_word_line(line);

                // Create a word from the parsed line
                Word {
                    content: word.to_string(),
                    meaning: meaning.unwrap_or("").to_string(),
                    class: Class::ClassifierNoun(ClassifierNoun {}),
                }
            })
            // Parse the proper nouns and add them
            .chain(
                defaults_parser::parse_str(include_str!("../proper_nouns.txt")).map(|line| {
                    let (word, metadata, meaning) = defaults_parser::parse_word_line(line);

                    let proper_noun = ProperNoun {
                        is_object: metadata.contains(&"OBJECT"),
                        is_subject: metadata.contains(&"SUBJECT"),
                    };

                    // Create a word from the parsed line
                    Word {
                        content: word.to_string(),
                        meaning: meaning.unwrap_or("").to_string(),
                        class: Class::ProperNoun(proper_noun),
                    }
                }),
            )
            // Parse the verbs and add them
            .chain(
                defaults_parser::parse_str(include_str!("../verbs.txt")).map(|line| {
                    let (word, _, meaning) = defaults_parser::parse_word_line(line);

                    // Create a word from the parsed line
                    Word {
                        content: word.to_string(),
                        meaning: meaning.unwrap_or("").to_string(),
                        class: Class::Verb(Verb {}),
                    }
                }),
            )
            // Parse the demonstratives and add them
            .chain(
                defaults_parser::parse_str(include_str!("../demonstratives.txt")).map(|line| {
                    let (word, _, meaning) = defaults_parser::parse_word_line(line);
                    // Create a word from the parsed line
                    Word {
                        content: word.to_string(),
                        meaning: meaning.unwrap_or("").to_string(),
                        class: Class::Demonstrative(Demonstrative {}),
                    }
                }),
            )
            // Parse the classifiers and add them
            .chain(
                defaults_parser::parse_str(include_str!("../classifiers.txt")).map(|line| {
                    let (word, _, meaning) = defaults_parser::parse_word_line(line);

                    // Create a word from the parsed line
                    Word {
                        content: word.to_string(),
                        meaning: meaning.unwrap_or("").to_string(),
                        class: Class::ClassifierNoun(ClassifierNoun {}),
                    }
                }),
            )
    }

    /// Get a random word belonging to a class.
    pub fn random_default<'a, R>(rng: &'a mut R, class: Class) -> Result<Word>
    where
        R: Rng,
    {
        debug!("Word: {:?}", class);
        Self::defaults()
            .filter(|word| word.class == class)
            .choose(rng)
            .ok_or(anyhow!("Could not get random word with class {:?}", class))
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

/// Classification of a word.
///
/// See:
/// - https://en.wikipedia.org/wiki/Part_of_speech
/// - https://en.wikipedia.org/wiki/Vietnamese_grammar
#[derive(Debug, Clone, Hash)]
pub enum Class {
    /// **Ý**: Italy.
    ProperNoun(ProperNoun),
    /// **Gái**: Girl.
    CommonNoun(CommonNoun),
    /// Ba **chiếc** áo dài: Three (sets of) áo dài.
    ClassifierNoun(ClassifierNoun),
    /// Tôi **đi**: I go.
    Verb(Verb),
    Adjective,
    Adverb,
    Pronoun,
    Conjunction,
    Interjection,
    Determiner,
    /// Ngày **kia**, ngày **kìa**, ngày **kía**, ngày **kịa**, ngày **kĩa**: On and on into the future.
    Demonstrative(Demonstrative),
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Class::ProperNoun(proper_noun) => {
                // Match proper nouns that are both subject or objects
                if let Class::ProperNoun(other) = other {
                    (proper_noun.is_object && other.is_object)
                        || (proper_noun.is_subject && other.is_subject)
                } else {
                    false
                }
            }
            // Only compare the variants, not the value
            _ => std::mem::discriminant(self) == std::mem::discriminant(other),
        }
    }
}

impl Eq for Class {}

/// Action, occurance or state of being.
///
/// Tôi **đi**: I go.
///
/// See: https://yourvietnamese.com/learn-vietnamese/vietnamese-verbs/
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Verb {}

/// Common noun subclasses.
///
/// **Gái**: Girl.
///
/// See: https://en.wikipedia.org/wiki/Vietnamese_grammar#Nouns_and_noun_phrases
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CommonNoun {
    Item,
    Collective,
    /// Or measure.
    Unit,
    Mass,
    Time,
    Abstract,
}

/// Usually names.
///
/// **Ý**: Italy.
///
/// See: https://en.wikipedia.org/wiki/Vietnamese_grammar#Nouns_and_noun_phrases
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ProperNoun {
    pub is_subject: bool,
    pub is_object: bool,
}

/// Classify a noun depending on the type of it's referent.
///
/// - Ba **chiếc** áo dài: Three (sets of) áo dài.
/// - Bán cho tôi bốn **con** gà: Sell me four chickens.
///
/// See:
/// - https://en.wikipedia.org/wiki/Vietnamese_grammar#Classifier_position
/// - https://en.wikipedia.org/wiki/Classifier_(linguistics)
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct ClassifierNoun {}

/// Noun modifier.
///
/// **Đây** đi chợ, **đấy** có đi không: I'm going to the market, what about you?
///
/// See: https://en.wikipedia.org/wiki/Vietnamese_grammar#Demonstratives
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct Demonstrative {}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_defaults() -> Result<()> {
        let mut default_words = Word::defaults();

        // Ensure there are default words
        assert!(default_words.next().is_some());

        Ok(())
    }
}
