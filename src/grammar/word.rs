/// A single word.
///
/// **Đi**: Go.
pub struct Word {
    /// The actual word as a string.
    content: String,
    /// How this word is classified.
    class: Class,
}

/// Classification of a word.
///
/// See:
/// - https://en.wikipedia.org/wiki/Part_of_speech
/// - https://en.wikipedia.org/wiki/Vietnamese_grammar
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
}

/// Common noun subclasses.
///
/// **Gái**: Girl.
///
/// See: https://en.wikipedia.org/wiki/Vietnamese_grammar#Nouns_and_noun_phrases
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
pub struct ClassifierNoun {
    /// Describing what type of nouns this classifies.
    description: String,
}
