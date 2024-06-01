use std::borrow::Cow;

mod among;
mod env;

pub mod algorithms;

/// Wrapps a usable interface around the actual stemmer implementation
pub struct SnowballStemmer {
    algorithm: algorithms::Algorithm,
}

impl SnowballStemmer {
    /// Create a new stemmer from an algorithm
    pub fn create(algorithm: algorithms::Algorithm) -> Self {
        SnowballStemmer { algorithm }
    }

    /// Stem a single word
    /// Please note, that the input is expected to be all lowercase (if that is applicable).
    pub fn stem<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut env = env::SnowballEnv::create(input);
        (self.algorithm)(&mut env);
        env.get_current()
    }
}
