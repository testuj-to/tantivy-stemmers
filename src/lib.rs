use std::borrow::Cow;
use std::mem;
use tantivy_tokenizer_api::{Token, TokenFilter, TokenStream, Tokenizer};

mod snowball;

pub mod algorithms;

/// Stemmer tokenizer. Several algorithms are supported, see [`algorithms`] or
/// https://crates.io/crates/tantivy-stemmers for a list of all available algorithms.
///
/// ❗️❗️ Tokens are expected to be lowercased beforehand.
#[derive(Clone)]
pub struct StemmerTokenizer {
    algorithm: algorithms::Algorithm,
}

impl StemmerTokenizer {
    /// Creates a new `StemmerTokenizer` [`StemmerTokenizer`] for a given language or variant algorithm.
    pub fn new(algorithm: algorithms::Algorithm) -> StemmerTokenizer {
        StemmerTokenizer { algorithm }
    }
}

#[cfg(feature = "english_porter_2")]
impl Default for StemmerTokenizer {
    /// Creates a new `StemmerTokenizer` [`StemmerTokenizer`] the default algorithm [`algorithms::english_porter_2`].
    fn default() -> Self {
        StemmerTokenizer::new(algorithms::english_porter_2)
    }
}

impl TokenFilter for StemmerTokenizer {
    type Tokenizer<T: Tokenizer> = StemmerFilter<T>;

    fn transform<T: Tokenizer>(self, tokenizer: T) -> StemmerFilter<T> {
        StemmerFilter {
            algorithm: self.algorithm,
            inner: tokenizer,
        }
    }
}

#[derive(Clone)]
pub struct StemmerFilter<T> {
    algorithm: algorithms::Algorithm,
    inner: T,
}

impl<T: Tokenizer> Tokenizer for StemmerFilter<T> {
    type TokenStream<'a> = StemmerTokenStream<T::TokenStream<'a>>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        StemmerTokenStream {
            tail: self.inner.token_stream(text),
            buffer: String::new(),
            algorithm: self.algorithm,
        }
    }
}

pub struct StemmerTokenStream<T> {
    tail: T,
    buffer: String,
    algorithm: algorithms::Algorithm,
}

impl<T: TokenStream> TokenStream for StemmerTokenStream<T> {
    fn advance(&mut self) -> bool {
        if !self.tail.advance() {
            return false;
        }

        let token = self.tail.token_mut();

        match (self.algorithm)(&token.text) {
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
