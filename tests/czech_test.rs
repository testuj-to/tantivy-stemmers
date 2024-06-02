
#[cfg(test)]
mod tests {
    use std::{fs::File, io::{BufRead, BufReader}};
    use tantivy::tokenizer::{LowerCaser, SimpleTokenizer, TextAnalyzer};
    use tantivy_tokenizer_api::TokenFilter;

    use tantivy_stemmers;
    use test_utils;

    #[test]
    fn it_stems_using_czech_domalic_aggressive() {
        let stemmer = tantivy_stemmers::StemmerTokenizer::new(
            tantivy_stemmers::algorithms::czech_dolamic_aggressive,
        );

        let chain = stemmer.transform(LowerCaser.transform(SimpleTokenizer::default()));
        let mut tokenizer = TextAnalyzer::builder(chain).build();

        let input_file = File::open("./tests/assets/cs/dolamic_aggressive.input.txt").unwrap();
        let output_file = File::open("./tests/assets/cs/dolamic_aggressive.output.txt").unwrap();

        let input_reader = BufReader::new(input_file);
        let output_reader = BufReader::new(output_file);

        let lines = input_reader.lines().zip(output_reader.lines());

        for (input, output) in lines {
            let input_str = input.unwrap();
            let mut token_stream = tokenizer.token_stream(input_str.as_str());

            assert_eq!(
                test_utils::reduce_token_stream_to_string(&mut token_stream),
                output.unwrap(),
            );
        }
    }
}