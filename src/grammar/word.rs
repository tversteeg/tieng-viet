use crate::grammar::defaults_parser;

/// A single word.
///
/// **Đi**: Go.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Word {
    /// The actual word as a string.
    content: String,
    /// How this word is classified.
    class: Class,
}

impl Word {
    /// Parse the included text files and generate a list of words from that.
    pub fn defaults() -> Vec<Word> {
        // Parse the classifiers and add them
        defaults_parser::parse_str(include_str!("../classifiers.txt"))
            .map(|line| {
                // Split on the first : symbol of the line and
                let (classifier, description) = line
                    .split_once(":")
                    .expect(&format!("Symbol ':' not found on line:\n\t{}", line));

                // Create a word from the parsed line
                Word {
                    content: classifier.to_string(),
                    class: Class::ClassifierNoun(ClassifierNoun::new(description.trim())),
                }
            })
            .collect()
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
    ProperNoun,
    /// **Gái**: Girl.
    CommonNoun(CommonNoun),
    /// Ba **chiếc** áo dài: Three (sets of) áo dài.
    ClassifierNoun(ClassifierNoun),
    /// Tôi **đi**: I go.
    ///
    /// See: https://yourvietnamese.com/learn-vietnamese/vietnamese-verbs/
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Conjunction,
    Interjection,
    Determiner,
    /// Ngày **kia**, ngày **kìa**, ngày **kía**, ngày **kịa**, ngày **kĩa**: On and on into the future.
    Demonstrative(Demonstrative),
}

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

/// Classify a noun depending on the type of it's referent.
///
/// - Ba **chiếc** áo dài: Three (sets of) áo dài.
/// - Bán cho tôi bốn **con** gà: Sell me four chickens.
///
/// See:
/// - https://en.wikipedia.org/wiki/Vietnamese_grammar#Classifier_position
/// - https://en.wikipedia.org/wiki/Classifier_(linguistics)
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ClassifierNoun {
    /// Describing what type of nouns this classifies.
    description: String,
}

impl ClassifierNoun {
    pub fn new<S>(description: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            description: description.into(),
        }
    }
}

/// Noun modifier.
///
/// **Đây** đi chợ, **đấy** có đi không: I'm going to the market, what about you
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
        let default_words = Word::defaults();

        assert!(default_words.len() > 0);

        Ok(())
    }
}
