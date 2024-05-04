use std::borrow::Cow;
use serde_derive::{Deserialize, Serialize};

mod among;
mod snowball_env;

pub mod algorithms;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum Algorithm {
    ArmenianMkrtchyan,
    Basque,
    Catalan,
    CzechDolamicAggressive,
    CzechDolamicLight,
    Danish,
    Dutch,
    EnglishLovins,
    EnglishPorter,
    EnglishPorter2,
    Finnish,
    French,
    German,
    Greek,
    HindiLightweight,
    Hungarian,
    IndonesianTala,
    IrishGaelic,
    Italian,
    LithuanianJocas,
    NorwegianBokmal,
    Portuguese,
    RomanianHeidelberg,
    RomanianTirdea,
    Romanian,
    Russian,
    Spanish,
    Swedish,
    TurkishCilden,
    YiddishUrieli,
}

/// Wrapps a usable interface around the actual stemmer implementation
pub struct Stemmer {
    stemmer: fn(&mut snowball_env::SnowballEnv) -> bool,
}

impl Stemmer {
    /// Create a new stemmer from an algorithm
    pub fn create(lang: Algorithm) -> Self {
        match lang {
            Algorithm::ArmenianMkrtchyan => Stemmer { stemmer: algorithms::armenian_mkrtchyan::stem },
            Algorithm::Basque => Stemmer { stemmer: algorithms::basque::stem },
            Algorithm::Catalan => Stemmer { stemmer: algorithms::catalan::stem },
            Algorithm::CzechDolamicAggressive => Stemmer { stemmer: algorithms::czech_dolamic_aggressive::stem },
            Algorithm::CzechDolamicLight => Stemmer { stemmer: algorithms::czech_dolamic_light::stem },
            Algorithm::Danish => Stemmer { stemmer: algorithms::danish::stem },
            Algorithm::Dutch => Stemmer { stemmer: algorithms::dutch::stem },
            Algorithm::EnglishLovins => Stemmer { stemmer: algorithms::english_lovins::stem },
            Algorithm::EnglishPorter => Stemmer { stemmer: algorithms::english_porter::stem },
            Algorithm::EnglishPorter2 => Stemmer { stemmer: algorithms::english_porter_2::stem },
            Algorithm::Finnish => Stemmer { stemmer: algorithms::finnish::stem },
            Algorithm::French => Stemmer { stemmer: algorithms::french::stem },
            Algorithm::German => Stemmer { stemmer: algorithms::german::stem },
            Algorithm::Greek => Stemmer { stemmer: algorithms::greek::stem },
            Algorithm::HindiLightweight => Stemmer { stemmer: algorithms::hindi_lightweight::stem },
            Algorithm::Hungarian => Stemmer { stemmer: algorithms::hungarian::stem },
            Algorithm::IndonesianTala => Stemmer { stemmer: algorithms::indonesian_tala::stem },
            Algorithm::IrishGaelic => Stemmer { stemmer: algorithms::irish_gaelic::stem },
            Algorithm::Italian => Stemmer { stemmer: algorithms::italian::stem },
            Algorithm::LithuanianJocas => Stemmer { stemmer: algorithms::lithuanian_jocas::stem },
            Algorithm::NorwegianBokmal => Stemmer { stemmer: algorithms::norwegian_bokmal::stem },
            Algorithm::Portuguese => Stemmer { stemmer: algorithms::portuguese::stem },
            Algorithm::RomanianHeidelberg => Stemmer { stemmer: algorithms::romanian_heidelberg::stem },
            Algorithm::RomanianTirdea => Stemmer { stemmer: algorithms::romanian_tirdea::stem },
            Algorithm::Romanian => Stemmer { stemmer: algorithms::romanian::stem },
            Algorithm::Russian => Stemmer { stemmer: algorithms::russian::stem },
            Algorithm::Spanish => Stemmer { stemmer: algorithms::spanish::stem },
            Algorithm::Swedish => Stemmer { stemmer: algorithms::swedish::stem },
            Algorithm::TurkishCilden => Stemmer { stemmer: algorithms::turkish_cilden::stem },
            Algorithm::YiddishUrieli => Stemmer { stemmer: algorithms::yiddish_urieli::stem },
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
