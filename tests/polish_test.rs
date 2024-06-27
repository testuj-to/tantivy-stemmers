
#[cfg(test)]
mod tests {
    use tantivy::tokenizer::{LowerCaser, SimpleTokenizer, TextAnalyzer};
    use tantivy_tokenizer_api::TokenFilter;

    use tantivy_stemmers;
    use test_utils;

    #[test]
    fn it_stems_using_polish_yarovoy() {
        let stemmer = tantivy_stemmers::StemmerTokenizer::new(
            tantivy_stemmers::algorithms::polish_yarovoy,
        );

        let chain = stemmer.transform(LowerCaser.transform(SimpleTokenizer::default()));
        let mut tokenizer = TextAnalyzer::builder(chain).build();

        let mut token_stream = tokenizer.token_stream(
            "Kariera na językach to wydarzenie zorganizowane z myślą o studentach i absolwentach znających języki obce na poziomie co najmniej Będą oni mieli okazję zastanowić się nad kierunkami rozwoju własnej kariery zawodowej w oparciu o informacje na temat możliwości wykorzystania swoich umiejętności lingwistycznych na współczesnym rynku pracy dlatego też nie chcę",
        );

        assert_eq!(
            test_utils::reduce_token_stream_to_string(&mut token_stream),
            "karier na język to wydarz zorganizowane z myśl o studen i absolwen znaj język obce na poziom co najmn będą oni miel okazj zastanow się nad kierunk rozwoj własn karier zawodow w opar o informacje na temat możliwośc wykorzys swoich umiejętnośc lingwistyczn na współczesnym rynk prac dlat też nie chcę",
        );
    }

    #[test]
    fn it_stems_using_polish_yarovoy_unaccented() {
        let stemmer = tantivy_stemmers::StemmerTokenizer::new(
            tantivy_stemmers::algorithms::polish_yarovoy_unaccented,
        );

        let chain = stemmer.transform(LowerCaser.transform(SimpleTokenizer::default()));
        let mut tokenizer = TextAnalyzer::builder(chain).build();

        let mut token_stream = tokenizer.token_stream(
            "Kariera na językach to wydarzenie zorganizowane z myślą o studentach i absolwentach znających języki obce na poziomie co najmniej Będą oni mieli okazję zastanowić się nad kierunkami rozwoju własnej kariery zawodowej w oparciu o informacje na temat możliwości wykorzystania swoich umiejętności lingwistycznych na współczesnym rynku pracy dlatego też nie chcę",
        );

        assert_eq!(
            test_utils::reduce_token_stream_to_string(&mut token_stream),
            "karier na jezyk to wydarz zorganizowan z mysl o studen i absolwen zna jezyk obce na poziom co najmn beda oni miel okazj zastan sie nad kierunk rozwoj wlasn karier zawod w opar o informacj na temat mozliwosc wykorzys swoich umiejetnosc lingwistyczn na wspolczesnym rynk prac dlat tez nie chce",
        );
    }
}
