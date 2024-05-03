use std::borrow::Cow;
use serde_derive::{Deserialize, Serialize};

mod among;
mod snowball_env;

pub mod algorithms;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum Algorithm {
    Catalan,
    CzechDolamicAggressive,
    CzechDolamicLight,
    EnglishPorter,
    EnglishPorter2,
    French,
    German,
    Portuguese,
    Spanish,
}

/// Wrapps a usable interface around the actual stemmer implementation
pub struct Stemmer {
    stemmer: fn(&mut snowball_env::SnowballEnv) -> bool,
}

impl Stemmer {
    /// Create a new stemmer from an algorithm
    pub fn create(lang: Algorithm) -> Self {
        match lang {
            Algorithm::Catalan => Stemmer { stemmer: algorithms::catalan::stem },
            Algorithm::CzechDolamicAggressive => Stemmer { stemmer: algorithms::czech_dolamic_aggressive::stem },
            Algorithm::CzechDolamicLight => Stemmer { stemmer: algorithms::czech_dolamic_light::stem },
            Algorithm::EnglishPorter => Stemmer { stemmer: algorithms::english_porter::stem },
            Algorithm::EnglishPorter2 => Stemmer { stemmer: algorithms::english_porter_2::stem },
            Algorithm::French => Stemmer { stemmer: algorithms::french::stem },
            Algorithm::German => Stemmer { stemmer: algorithms::german::stem },
            Algorithm::Portuguese => Stemmer { stemmer: algorithms::portuguese::stem },
            Algorithm::Spanish => Stemmer { stemmer: algorithms::spanish::stem },
        }
    }

    /// Stem a single word
    /// Please note, that the input is expected to be all lowercase (if that is applicable).
    pub fn stem<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut env = snowball_env::SnowballEnv::create(input);
        (self.stemmer)(&mut env);
        env.get_current()
    }
}
