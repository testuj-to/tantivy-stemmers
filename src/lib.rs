use std::borrow::Cow;
use std::mem;
use serde_derive::{Deserialize, Serialize};
use tantivy_tokenizer_api::{Token, TokenFilter, TokenStream, Tokenizer};

mod snowball;

/// Available stemmer languages and specific algorithms.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum Algorithm {
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
    Hungarian,
    IrishGaelic,
    Italian,
    NorwegianBokmal,
    Portuguese,
    RomanianHeidelberg,
    RomanianTirdea,
    Romanian,
    Russian,
    Spanish,
    Swedish,
    TurkishCilden,
}

impl Algorithm {
    fn snowball_algorithm(self) -> snowball::Algorithm {
        use self::Algorithm::*;

        match self {
            Basque => snowball::Algorithm::Basque,
            Catalan => snowball::Algorithm::Catalan,
            CzechDolamicAggressive => snowball::Algorithm::CzechDolamicAggressive,
            CzechDolamicLight => snowball::Algorithm::CzechDolamicLight,
            Danish => snowball::Algorithm::Danish,
            Dutch => snowball::Algorithm::Dutch,
            EnglishLovins => snowball::Algorithm::EnglishLovins,
            EnglishPorter => snowball::Algorithm::EnglishPorter,
            EnglishPorter2 => snowball::Algorithm::EnglishPorter2,
            Finnish => snowball::Algorithm::Finnish,
            French => snowball::Algorithm::French,
            German => snowball::Algorithm::German,
            Hungarian => snowball::Algorithm::Hungarian,
            IrishGaelic => snowball::Algorithm::IrishGaelic,
            Italian => snowball::Algorithm::Italian,
            NorwegianBokmal => snowball::Algorithm::NorwegianBokmal,
            Portuguese => snowball::Algorithm::Portuguese,
            RomanianHeidelberg => snowball::Algorithm::RomanianHeidelberg,
            RomanianTirdea => snowball::Algorithm::RomanianTirdea,
            Romanian => snowball::Algorithm::Romanian,
            Russian => snowball::Algorithm::Russian,
            Spanish => snowball::Algorithm::Spanish,
            Swedish => snowball::Algorithm::Swedish,
            TurkishCilden => snowball::Algorithm::TurkishCilden,
        }
    }
}

/// `Stemmer` token filter. Several languages are supported, see [`Language`] for the available
/// languages.
/// Tokens are expected to be lowercased beforehand.
#[derive(Clone)]
pub struct Stemmer {
    stemmer_algorithm: snowball::Algorithm,
}

impl Stemmer {
    /// Creates a new `Stemmer` [`TokenFilter`] for a given language algorithm.
    pub fn new(algorithm: Algorithm) -> Stemmer {
        Stemmer {
            stemmer_algorithm: algorithm.snowball_algorithm(),
        }
    }
}

impl Default for Stemmer {
    /// Creates a new `Stemmer` [`TokenFilter`] for [`Algorithm::EnglishPorter`].
    fn default() -> Self {
        Stemmer::new(Algorithm::EnglishPorter)
    }
}

impl TokenFilter for Stemmer {
    type Tokenizer<T: Tokenizer> = StemmerFilter<T>;

    fn transform<T: Tokenizer>(self, tokenizer: T) -> StemmerFilter<T> {
        StemmerFilter {
            stemmer_algorithm: self.stemmer_algorithm,
            inner: tokenizer,
        }
    }
}

#[derive(Clone)]
pub struct StemmerFilter<T> {
    stemmer_algorithm: snowball::Algorithm,
    inner: T,
}

impl<T: Tokenizer> Tokenizer for StemmerFilter<T> {
    type TokenStream<'a> = StemmerTokenStream<T::TokenStream<'a>>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        StemmerTokenStream {
            tail: self.inner.token_stream(text),
            buffer: String::new(),
            stemmer: snowball::Stemmer::create(self.stemmer_algorithm),
        }
    }
}

pub struct StemmerTokenStream<T> {
    tail: T,
    buffer: String,
    stemmer: snowball::Stemmer,
}

impl<T: TokenStream> TokenStream for StemmerTokenStream<T> {
    fn advance(&mut self) -> bool {
        if !self.tail.advance() {
            return false;
        }

        let token = self.tail.token_mut();

        match self.stemmer.stem(&token.text) {
            Cow::Owned(stemmed_str) => token.text = stemmed_str,
            Cow::Borrowed(stemmed_str) => {
                self.buffer.clear();
                self.buffer.push_str(stemmed_str);
                mem::swap(&mut token.text, &mut self.buffer);
            }
        }

        true
    }

    fn token(&self) -> &Token {
        self.tail.token()
    }

    fn token_mut(&mut self) -> &mut Token {
        self.tail.token_mut()
    }
}
