
#[cfg(test)]
mod tests {
    use tantivy::tokenizer::{LowerCaser, SimpleTokenizer, TextAnalyzer};
    use tantivy_tokenizer_api::{BoxTokenStream, TokenFilter};
    use tantivy_stemmers;

    fn reduce_token_stream_to_string(token_stream: &mut BoxTokenStream) -> String {
        let mut tokens: Vec<String> = Vec::new();

        loop {
            match token_stream.next() {
                Some(token) => {
                    tokens.push(token.text.clone());
                },
                None => break,
            }
        }

        return tokens.join(" ");
    }

    #[test]
    fn it_creates_default_stemmer() {
        let stemmer = tantivy_stemmers::StemmerTokenizer::default();

        let chain = stemmer.transform(LowerCaser.transform(SimpleTokenizer::default()));
        let mut tokenizer = TextAnalyzer::builder(chain).build();

        let mut token_stream = tokenizer.token_stream("HOW inSAnElY qUEsTiOnaBLE");

        let result = reduce_token_stream_to_string(&mut token_stream);

        assert_eq!(result, "how insan question");
    }

    #[test]
    fn it_creates_non_default_stemmer() {
        let stemmer = tantivy_stemmers::StemmerTokenizer::new(
            tantivy_stemmers::algorithms::czech_dolamic_aggressive,
        );

        let chain = stemmer.transform(LowerCaser.transform(SimpleTokenizer::default()));
        let mut tokenizer = TextAnalyzer::builder(chain).build();

        let mut token_stream = tokenizer.token_stream("Bez dlouhé prodlevy");

        let result = reduce_token_stream_to_string(&mut token_stream);

        assert_eq!(result, "bez dlouh prodlev");
    }
}
