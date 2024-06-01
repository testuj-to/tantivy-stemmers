
use super::env::SnowballEnv;

pub type Algorithm = fn(&mut SnowballEnv) -> bool;


// arabic

#[cfg(feature = "arabic")]
mod arabic;

#[cfg(feature = "arabic")]
pub fn arabic(env: &mut SnowballEnv) -> bool {
    return arabic::stem(env);
}


// armenian_mkrtchyan

#[cfg(feature = "armenian_mkrtchyan")]
mod armenian_mkrtchyan;

#[cfg(feature = "armenian_mkrtchyan")]
pub fn armenian_mkrtchyan(env: &mut SnowballEnv) -> bool {
    return armenian_mkrtchyan::stem(env);
}


// basque

#[cfg(feature = "basque")]
mod basque;

#[cfg(feature = "basque")]
pub fn basque(env: &mut SnowballEnv) -> bool {
    return basque::stem(env);
}


// catalan

#[cfg(feature = "catalan")]
mod catalan;

#[cfg(feature = "catalan")]
pub fn catalan(env: &mut SnowballEnv) -> bool {
    return catalan::stem(env);
}


// czech_dolamic_aggressive

#[cfg(feature = "czech_dolamic_aggressive")]
mod czech_dolamic_aggressive;

#[cfg(feature = "czech_dolamic_aggressive")]
pub fn czech_dolamic_aggressive(env: &mut SnowballEnv) -> bool {
    return czech_dolamic_aggressive::stem(env);
}


// czech_dolamic_light

#[cfg(feature = "czech_dolamic_light")]
mod czech_dolamic_light;

#[cfg(feature = "czech_dolamic_light")]
pub fn czech_dolamic_light(env: &mut SnowballEnv) -> bool {
    return czech_dolamic_light::stem(env);
}


// danish

#[cfg(feature = "danish")]
mod danish;

#[cfg(feature = "danish")]
pub fn danish(env: &mut SnowballEnv) -> bool {
    return danish::stem(env);
}


// dutch

#[cfg(feature = "dutch")]
mod dutch;

#[cfg(feature = "dutch")]
pub fn dutch(env: &mut SnowballEnv) -> bool {
    return dutch::stem(env);
}


// english_lovins

#[cfg(feature = "english_lovins")]
mod english_lovins;

#[cfg(feature = "english_lovins")]
pub fn english_lovins(env: &mut SnowballEnv) -> bool {
    return english_lovins::stem(env);
}


// english_porter

#[cfg(feature = "english_porter")]
mod english_porter;

#[cfg(feature = "english_porter")]
pub fn english_porter(env: &mut SnowballEnv) -> bool {
    return english_porter::stem(env);
}


// english_porter_2

#[cfg(feature = "english_porter_2")]
mod english_porter_2;

#[cfg(feature = "english_porter_2")]
pub fn english_porter_2(env: &mut SnowballEnv) -> bool {
    return english_porter_2::stem(env);
}


// estonian_freienthal

#[cfg(feature = "estonian_freienthal")]
mod estonian_freienthal;

#[cfg(feature = "estonian_freienthal")]
pub fn estonian_freienthal(env: &mut SnowballEnv) -> bool {
    return estonian_freienthal::stem(env);
}


// finnish

#[cfg(feature = "finnish")]
mod finnish;

#[cfg(feature = "finnish")]
pub fn finnish(env: &mut SnowballEnv) -> bool {
    return finnish::stem(env);
}


// french

#[cfg(feature = "french")]
mod french;

#[cfg(feature = "french")]
pub fn french(env: &mut SnowballEnv) -> bool {
    return french::stem(env);
}


// german

#[cfg(feature = "german")]
mod german;

#[cfg(feature = "german")]
pub fn german(env: &mut SnowballEnv) -> bool {
    return german::stem(env);
}


// greek

#[cfg(feature = "greek")]
mod greek;

#[cfg(feature = "greek")]
pub fn greek(env: &mut SnowballEnv) -> bool {
    return greek::stem(env);
}


// hindi_lightweight

#[cfg(feature = "hindi_lightweight")]
mod hindi_lightweight;

#[cfg(feature = "hindi_lightweight")]
pub fn hindi_lightweight(env: &mut SnowballEnv) -> bool {
    return hindi_lightweight::stem(env);
}


// hungarian

#[cfg(feature = "hungarian")]
mod hungarian;

#[cfg(feature = "hungarian")]
pub fn hungarian(env: &mut SnowballEnv) -> bool {
    return hungarian::stem(env);
}


// indonesian_tala

#[cfg(feature = "indonesian_tala")]
mod indonesian_tala;

#[cfg(feature = "indonesian_tala")]
pub fn indonesian_tala(env: &mut SnowballEnv) -> bool {
    return indonesian_tala::stem(env);
}


// irish_gaelic

#[cfg(feature = "irish_gaelic")]
mod irish_gaelic;

#[cfg(feature = "irish_gaelic")]
pub fn irish_gaelic(env: &mut SnowballEnv) -> bool {
    return irish_gaelic::stem(env);
}


// italian

#[cfg(feature = "italian")]
mod italian;

#[cfg(feature = "italian")]
pub fn italian(env: &mut SnowballEnv) -> bool {
    return italian::stem(env);
}


// lithuanian_jocas

#[cfg(feature = "lithuanian_jocas")]
mod lithuanian_jocas;

#[cfg(feature = "lithuanian_jocas")]
pub fn lithuanian_jocas(env: &mut SnowballEnv) -> bool {
    return lithuanian_jocas::stem(env);
}


// nepali

#[cfg(feature = "nepali")]
mod nepali;

#[cfg(feature = "nepali")]
pub fn nepali(env: &mut SnowballEnv) -> bool {
    return nepali::stem(env);
}


// norwegian_bokmal

#[cfg(feature = "norwegian_bokmal")]
mod norwegian_bokmal;

#[cfg(feature = "norwegian_bokmal")]
pub fn norwegian_bokmal(env: &mut SnowballEnv) -> bool {
    return norwegian_bokmal::stem(env);
}


// portuguese

#[cfg(feature = "portuguese")]
mod portuguese;

#[cfg(feature = "portuguese")]
pub fn portuguese(env: &mut SnowballEnv) -> bool {
    return portuguese::stem(env);
}


// romanian_heidelberg

#[cfg(feature = "romanian_heidelberg")]
mod romanian_heidelberg;

#[cfg(feature = "romanian_heidelberg")]
pub fn romanian_heidelberg(env: &mut SnowballEnv) -> bool {
    return romanian_heidelberg::stem(env);
}


// romanian_tirdea

#[cfg(feature = "romanian_tirdea")]
mod romanian_tirdea;

#[cfg(feature = "romanian_tirdea")]
pub fn romanian_tirdea(env: &mut SnowballEnv) -> bool {
    return romanian_tirdea::stem(env);
}


// romanian

#[cfg(feature = "romanian")]
mod romanian;

#[cfg(feature = "romanian")]
pub fn romanian(env: &mut SnowballEnv) -> bool {
    return romanian::stem(env);
}


// russian

#[cfg(feature = "russian")]
mod russian;

#[cfg(feature = "russian")]
pub fn russian(env: &mut SnowballEnv) -> bool {
    return russian::stem(env);
}


// spanish

#[cfg(feature = "spanish")]
mod spanish;

#[cfg(feature = "spanish")]
pub fn spanish(env: &mut SnowballEnv) -> bool {
    return spanish::stem(env);
}


// swedish

#[cfg(feature = "swedish")]
mod swedish;

#[cfg(feature = "swedish")]
pub fn swedish(env: &mut SnowballEnv) -> bool {
    return swedish::stem(env);
}


// turkish_cilden

#[cfg(feature = "turkish_cilden")]
mod turkish_cilden;

#[cfg(feature = "turkish_cilden")]
pub fn turkish_cilden(env: &mut SnowballEnv) -> bool {
    return turkish_cilden::stem(env);
}


// yiddish_urieli

#[cfg(feature = "yiddish_urieli")]
mod yiddish_urieli;

#[cfg(feature = "yiddish_urieli")]
pub fn yiddish_urieli(env: &mut SnowballEnv) -> bool {
    return yiddish_urieli::stem(env);
}

