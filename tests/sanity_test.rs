
#[cfg(test)]
mod tests {
    use tantivy::tokenizer::{LowerCaser, SimpleTokenizer, TextAnalyzer};
    use tantivy_tokenizer_api::TokenFilter;

    use tantivy_stemmers;
    use test_utils;

    #[test]
    fn it_creates_default_stemmer() {
        let stemmer = tantivy_stemmers::StemmerTokenizer::default();

        let chain = stemmer.transform(LowerCaser.transform(SimpleTokenizer::default()));
        let mut tokenizer = TextAnalyzer::builder(chain).build();

        let mut token_stream = tokenizer.token_stream("HOW inSAnElY qUEsTiOnaBLE");

        assert_eq!(
            test_utils::reduce_token_stream_to_string(&mut token_stream),
            "how insan question",
        );
    }

    #[test]
    fn it_creates_non_default_stemmer() {
        let stemmer = tantivy_stemmers::StemmerTokenizer::new(
            tantivy_stemmers::algorithms::czech_dolamic_aggressive,
        );

        let chain = stemmer.transform(LowerCaser.transform(SimpleTokenizer::default()));
        let mut tokenizer = TextAnalyzer::builder(chain).build();

        let mut token_stream = tokenizer.token_stream("Bez dlouhé prodlevy");

        assert_eq!(
            test_utils::reduce_token_stream_to_string(&mut token_stream),
            "bez dlouh prodlev",
        );
    }
}
