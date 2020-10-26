use crate::grammar::defaults_parser;

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
                // Split on the first : symbol of the line
                let (classifier, meaning) = line
                    .split_once(":")
                    .expect(&format!("Symbol ':' not found on line:\n\t{}", line));

                // Create a word from the parsed line
                Word {
                    content: classifier.to_string(),
                    meaning: meaning.trim().to_string(),
                    class: Class::ClassifierNoun(ClassifierNoun {}),
                }
            })
            // Parse the proper nouns and add them
            .chain(
                defaults_parser::parse_str(include_str!("../proper_nouns.txt")).map(|line| {
                    // Split on the first : symbol of the line
                    let (noun, meaning) = line
                        .split_once(":")
                        .expect(&format!("Symbol ':' not found on line:\n\t{}", line));

                    // Create a word from the parsed line
                    Word {
                        content: noun.to_string(),
                        meaning: meaning.trim().to_string(),
                        class: Class::ProperNoun(ProperNoun {}),
                    }
                }),
            )
            // Parse the verbs and add them
            .chain(
                defaults_parser::parse_str(include_str!("../verbs.txt")).map(|line| {
                    // Split on the first : symbol of the line
                    let (verb, meaning) = line
                        .split_once(":")
                        .expect(&format!("Symbol ':' not found on line:\n\t{}", line));

                    // Create a word from the parsed line
                    Word {
                        content: verb.to_string(),
                        meaning: meaning.trim().to_string(),
                        class: Class::Verb(Verb {}),
                    }
                }),
            )
    }
}

/// Classification of a word.
///
/// See:
/// - https://en.wikipedia.org/wiki/Part_of_speech
/// - https://en.wikipedia.org/wiki/Vietnamese_grammar
#[derive(Debug, Clone, PartialEq, Hash)]
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

/// Action, occurance or state of being.
///
/// Tôi **đi**: I go.
///
/// See: https://yourvietnamese.com/learn-vietnamese/vietnamese-verbs/
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ProperNoun {}

/// Classify a noun depending on the type of it's referent.
///
/// - Ba **chiếc** áo dài: Three (sets of) áo dài.
/// - Bán cho tôi bốn **con** gà: Sell me four chickens.
///
/// See:
/// - https://en.wikipedia.org/wiki/Vietnamese_grammar#Classifier_position
/// - https://en.wikipedia.org/wiki/Classifier_(linguistics)
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ClassifierNoun {}

/// Noun modifier.
///
/// **Đây** đi chợ, **đấy** có đi không: I'm going to the market, what about you?
///
/// See: https://en.wikipedia.org/wiki/Vietnamese_grammar#Demonstratives
#[derive(Debug, Clone, PartialEq, Hash)]
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
