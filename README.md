
# Tantivy stemmers (tokenizer)

This library bundles several OSS to provide a collection of stemming algorithms for various languages as a Tantivy tokenizer. [Tantivy](https://github.com/quickwit-oss/tantivy) is a full-text search engine library written in Rust. As its default `Stemmer` tokenizer depends on a dead library `rust-stemmers`, there are only a very few languages available by default. Nevertheless, Tantivy provides an easy way to build our own custom tokenizers (see the [tantivy-tokenizer-api](https://crates.io/crates/tantivy-tokenizer-api) for details).

This library is a compilation of several OSS projects into 1 library:
- [`rust-stemmers`](https://github.com/CurrySoftware/rust-stemmers)

  Library `rust-stemmers` (used by Tantivy under the hood) implements a Rust interface for a Snowball algorithms of several languages. This library is inspired by `rust-stemmers` and some source code is taken directly from `rust-stemmers` (namely `src/snowball/*`).
- [`Tantivy`](https://github.com/quickwit-oss/tantivy)

  Implementation of the `Stemmer` in this library is pretty much a copy of the original implementation of the [`Stemmer` tokenizer](https://github.com/quickwit-oss/tantivy/blob/main/src/tokenizer/stemmer.rs) in the Tantivy library. Only this lib does not depend on the `rust-stemmers` package and instead includes various algorithms in it self. And instead of importing from the `tantivy` lib, this library imports from `tantivy-tokenizer-api`.
- **Algorithms**

  Most, if not all, stemming algorithms are obtained from the official [Snowball website](https://snowballstem.org/) and compiled using the Snowball compiler into Rust. More information about individual algorithm licenses are noted below - most are published under the BSD license.

## Usage

```rust
use tantivy::Index;
use tantivy::schema::{Schema, TextFieldIndexing, TextOptions, IndexRecordOption};
use tantivy::tokenizer::{LowerCaser, SimpleTokenizer, TextAnalyzer};
use tantivy_stemmers;

fn main() {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field(
        "title",
        TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    // Set name of the tokenizer, we will register it shortly
                    .set_tokenizer("lang_cs")
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
            )
            .set_stored(),
    );

    let schema = schema_builder.build();
    let index = Index::create_in_ram(schema.clone());

    // Create an instance of the Stemmer

    // With default algorithm (default algorithm is EnglishPorter)
    // let stemmer = tantivy_stemmers::Stemmer::default();

    // With a specific algorithm
    let stemmer = tantivy_stemmers::Stemmer::new(
        tantivy_stemmers::Algorithm::CzechDolamicAggressive,
    );

    // Before we register it, we need to wrap it in an instance
    // of the TextAnalyzer tokenizer. We also have to transform
    // the text to lowercase since our stemmer expects lowercase.
    let tokenizer = TextAnalyzer::builder(
        stemmer.transform(LowerCaser.transform(SimpleTokenizer::default())),
    ).build();

    // Register our tokenizer with Tantivy under a custom name
    index.tokenizers().register("lang_cs", tokenizer);
}
```

## Supported algorithms

- **Armenian**

  The Armenian Snowball algorithm was developed by **Astghik Mkrtchyan** and source code has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/armenian/stemmer.html).

- **Basque**

  The Basque Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/basque/stemmer.html).

- **Catalan**

  The Catalan Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/catalan/stemmer.html).

- **Czech**

  Currently only a single algorithm (in an `aggressive` and `light` variants) is available: `Dolamic`. This algorithm has been developed by **Ljiljana Dolamic** & Jacques Savoy and published under the BSD license. It's written in the [Snowball language](https://snowballstem.org/) and is available on the [Snowball website](https://snowballstem.org/algorithms/czech/stemmer.html).

  *There is 1 more stemming algorithm for the Czech language: `Hellebrand`. This algorithm has been developed by **David Hellebrand** & Petr Chmelař. It's also written in the Snowball language and is available as a [Master's thesis here](https://www.fit.vut.cz/research/product/133). However, this algorithm has been published under the GNU license and **is therefore not included in this library as we'd like to keep the BSD license** on this library. (If you wish, you can always compile the `Hellebrand` algorithm from Snowball to Rust and include it yourself.)*

  ##### Personal note - apology for our countrymen

  While researching the internet for a Czech stemming algorithms, I have found only 2, out of which only 1 with a suitable license - the `Dolamic` algorithm. However, I have found many references to this algorithm - both international and domestic. My blood boiled when I noticed a curious trend: while every single international reference to this algorithm credited the actual author (**Ljiljana Dolamic**), every single domestic reference credited her supervisor (Jacques Savoy) and didn't even bother to mention her. Czech is an especially difficult language even for native speakers, let alone for a foreign researcher, let alone if they're building a liguistics algorithm used by those native speakers to this day (looking at you PostgreSQL and ElasticSearch). Therefore I'd like to apologize for the lack of given damn from my countrymen to the effort this algorithm took from its author - Ljiljana Dolamic.

- **Danish**

  The Danish Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/danish/stemmer.html).

- **Dutch**

  The Dutch Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/dutch/stemmer.html).

- **English**

  Three english algorithms in Snowball are available from the official Snowball website - the Porter, Porter 2 and Lovins. (At least) the first two algorithms have been developed by Dr. Martin Porter. **The Porter algorithm (original) is used as a default algorithm in this library.** If you wish, you can specify to use the newer Porter 2 algorithm (`Algorithm::EnglishPorter2`) or the Lovins algorithm (`Algorithm::EnglishLovins`).

- **Finnish**

  The Finnish Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/finnish/stemmer.html).

- **French**

  The French Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/french/stemmer.html).

- **German**

  The German Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/german/stemmer.html).

- **Greek**

  The Greek Snowball algorithm has been developed by **Georgios Ntais** in 2006 and later enhanced by **Spyridon Saroukos** in 2008. The source code has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/greek/stemmer.html).

- **Hindi**

  The Hindi (lightweight) Snowball algorithm was developed by **A. Ramanathan** and **D. Rao** in 2003. Its source code has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/hindi/stemmer.html).

- **Hungarian**

  The Hungarian Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/hungarian/stemmer.html).

- **Indonesian**

  The Indonesian Snowball algorithm was developed by **Fadillah Z. Tala** in 2003 and source codes has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/indonesian/stemmer.html).

- **Irish (Gaelic)**

  The Irish (Gaelic) Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/irish/stemmer.html).

- **Italian**

  The Italian Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/italian/stemmer.html).

- **Lithuanian**

  The Lithuanian Snowball algorithm (`LithuanianJocas`) was contributed by Dainius Jocas. Its source code has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/lithuanian/stemmer.html).

- **Norwegian (Bokmål)**

  The Norwegian (Bokmål variant) Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/norwegian/stemmer.html).

- **Portuguese**

  The Portuguese Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/portuguese/stemmer.html).

- **Romanian**

  Three Snowball algorithms for the Romainian language are available: `Romanian`, `RomanianHeidelberg` and `RomanianTirdea`. All algorithm were obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/romanian/stemmer.html) and [Snowball website](https://snowballstem.org/otherapps/romanian/).

  The `RomanianHeidelberg` algorithm has been developed in 2006 by **Marina Stegarescu**, **Doina Gliga** and **Erwin Glockner** at the Ruprecht-Karls-University of Heidelberg (Department of Computational Linguistics).

  The `RomanianTirdea` has been developed in 2006 by **Irina Tirdea**.

- **Russian**

  The Russian Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/russian/stemmer.html).

- **Spanish**

  The Spanish Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/spanish/stemmer.html).

- **Swedish**

  The Swedish Snowball algorithm was obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/swedish/stemmer.html).

- **Turkish**

  The Turkish Snowball algorithm was developed by **Evren (Kapusuz) Çilden** in 2007. The source code has been obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/turkish/stemmer.html).

  ##### Note from the Snowball website

  > The Turkish stemming algorithm was provided by Evren Kapusuz Cilden. It stems only noun and nominal verb suffixes because noun stems are more important for information retrieval, and only handling these simplifies the algorithm significantly.

  > In her paper (linked above) Evren explains

  > The stemmer can be enhanced to stem all kinds of verb suffixes. In Turkish, there are over fifty suffixes that can be affixed to verbs [2]. The morphological structure of verb suffixes is more complicated than noun suffixes. Despite this, one can use the methodology presented in this paper to enhance the stemmer to find stems of all kinds of Turkish words.

- **Yiddish**

  The Yiddish Snowball algorithm was created by Assaf Urieli in 2020 and obtained under the BSD license from the official [Snowball website](https://snowballstem.org/algorithms/yiddish/stemmer.html).
