use std::borrow::Cow;

use super::snowball;

pub type Algorithm = fn(&str) -> Cow<str>;

// Snowball algorithms

fn stem_with_snowball<'a>(stem: fn(&mut snowball::env::SnowballEnv) -> bool, input: &'a str) -> Cow<'a, str> {
    let mut env = snowball::env::SnowballEnv::create(input);
    stem(&mut env);
    env.get_current()
}

#[cfg(feature = "arabic")]
pub fn arabic<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::arabic, input)
}

#[cfg(feature = "armenian_mkrtchyan")]
pub fn armenian_mkrtchyan<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::armenian_mkrtchyan, input)
}

#[cfg(feature = "basque")]
pub fn basque<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::basque, input)
}

#[cfg(feature = "catalan")]
pub fn catalan<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::catalan, input)
}

#[cfg(feature = "czech_dolamic_aggressive")]
pub fn czech_dolamic_aggressive<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::czech_dolamic_aggressive, input)
}

#[cfg(feature = "czech_dolamic_light")]
pub fn czech_dolamic_light<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::czech_dolamic_light, input)
}

#[cfg(feature = "danish")]
pub fn danish<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::danish, input)
}

#[cfg(feature = "dutch")]
pub fn dutch<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::dutch, input)
}

#[cfg(feature = "english_lovins")]
pub fn english_lovins<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::english_lovins, input)
}

#[cfg(feature = "english_porter")]
pub fn english_porter<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::english_porter, input)
}

#[cfg(feature = "english_porter_2")]
pub fn english_porter_2<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::english_porter_2, input)
}

#[cfg(feature = "estonian_freienthal")]
pub fn estonian_freienthal<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::estonian_freienthal, input)
}

#[cfg(feature = "finnish")]
pub fn finnish<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::finnish, input)
}

#[cfg(feature = "french")]
pub fn french<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::french, input)
}

#[cfg(feature = "german")]
pub fn german<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::german, input)
}

#[cfg(feature = "greek")]
pub fn greek<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::greek, input)
}

#[cfg(feature = "hindi_lightweight")]
pub fn hindi_lightweight<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::hindi_lightweight, input)
}

#[cfg(feature = "hungarian")]
pub fn hungarian<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::hungarian, input)
}

#[cfg(feature = "indonesian_tala")]
pub fn indonesian_tala<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::indonesian_tala, input)
}

#[cfg(feature = "irish_gaelic")]
pub fn irish_gaelic<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::irish_gaelic, input)
}

#[cfg(feature = "italian")]
pub fn italian<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::italian, input)
}

#[cfg(feature = "lithuanian_jocas")]
pub fn lithuanian_jocas<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::lithuanian_jocas, input)
}

#[cfg(feature = "nepali")]
pub fn nepali<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::nepali, input)
}

#[cfg(feature = "norwegian_bokmal")]
pub fn norwegian_bokmal<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::norwegian_bokmal, input)
}

#[cfg(feature = "portuguese")]
pub fn portuguese<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::portuguese, input)
}

#[cfg(feature = "romanian_heidelberg")]
pub fn romanian_heidelberg<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::romanian_heidelberg, input)
}

#[cfg(feature = "romanian_tirdea")]
pub fn romanian_tirdea<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::romanian_tirdea, input)
}

#[cfg(feature = "romanian")]
pub fn romanian<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::romanian, input)
}

#[cfg(feature = "russian")]
pub fn russian<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::russian, input)
}

#[cfg(feature = "spanish")]
pub fn spanish<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::spanish, input)
}

#[cfg(feature = "swedish")]
pub fn swedish<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::swedish, input)
}

#[cfg(feature = "turkish_cilden")]
pub fn turkish_cilden<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::turkish_cilden, input)
}

#[cfg(feature = "yiddish_urieli")]
pub fn yiddish_urieli<'a>(input: &'a str) -> Cow<'a, str> {
    stem_with_snowball(snowball::algorithms::yiddish_urieli, input)
}

