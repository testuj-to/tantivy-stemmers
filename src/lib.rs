use std::borrow::Cow;
use std::mem;
use serde_derive::{Deserialize, Serialize};
use tantivy_tokenizer_api::{Token, TokenFilter, TokenStream, Tokenizer};

mod snowball;

/// Available stemmer languages and specific algorithms.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum Algorithm {
    CzechDolamicAggressive,
    CzechDolamicLight,
    EnglishPorter,
    French,
    German,
}

impl Algorithm {
    fn snowball_algorithm(self) -> snowball::Algorithm {
        use self::Algorithm::*;

        match self {
            CzechDolamicAggressive => snowball::Algorithm::CzechDolamicAggressive,
            CzechDolamicLight => snowball::Algorithm::CzechDolamicLight,
            EnglishPorter => snowball::Algorithm::EnglishPorter,
            French => snowball::Algorithm::French,
            German => snowball::Algorithm::German,
        }
    }
}

/// `Stemmer` token filter for the Czech language, see [`Algorithm`] for the available Algorithms.
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
