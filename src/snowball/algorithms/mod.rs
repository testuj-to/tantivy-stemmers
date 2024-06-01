
use super::snowball_env::SnowballEnv;

pub type Algorithm = fn(&mut SnowballEnv) -> bool;


// Arabic

#[cfg(feature = "arabic")]
mod arabic;

#[cfg(feature = "arabic")]
pub fn arabic(env: &mut SnowballEnv) -> bool {
    return arabic::stem(env);
}


// ArmenianMkrtchyan

#[cfg(feature = "armenian-mkrtchyan")]
mod armenian_mkrtchyan;

#[cfg(feature = "armenian-mkrtchyan")]
pub fn armenian_mkrtchyan(env: &mut SnowballEnv) -> bool {
    return armenian_mkrtchyan::stem(env);
}


// Basque

#[cfg(feature = "basque")]
mod basque;

#[cfg(feature = "basque")]
pub fn basque(env: &mut SnowballEnv) -> bool {
    return basque::stem(env);
}


// Catalan

#[cfg(feature = "catalan")]
mod catalan;

#[cfg(feature = "catalan")]
pub fn catalan(env: &mut SnowballEnv) -> bool {
    return catalan::stem(env);
}


// CzechDolamicAggressive

#[cfg(feature = "czech-dolamic-aggressive")]
mod czech_dolamic_aggressive;

#[cfg(feature = "czech-dolamic-aggressive")]
pub fn czech_dolamic_aggressive(env: &mut SnowballEnv) -> bool {
    return czech_dolamic_aggressive::stem(env);
}


// CzechDolamicLight

#[cfg(feature = "czech-dolamic-light")]
mod czech_dolamic_light;

#[cfg(feature = "czech-dolamic-light")]
pub fn czech_dolamic_light(env: &mut SnowballEnv) -> bool {
    return czech_dolamic_light::stem(env);
}


// Danish

#[cfg(feature = "danish")]
mod danish;

#[cfg(feature = "danish")]
pub fn danish(env: &mut SnowballEnv) -> bool {
    return danish::stem(env);
}


// Dutch

#[cfg(feature = "dutch")]
mod dutch;

#[cfg(feature = "dutch")]
pub fn dutch(env: &mut SnowballEnv) -> bool {
    return dutch::stem(env);
}


// EnglishLovins

#[cfg(feature = "english-lovins")]
mod english_lovins;

#[cfg(feature = "english-lovins")]
pub fn english_lovins(env: &mut SnowballEnv) -> bool {
    return english_lovins::stem(env);
}


// EnglishPorter

#[cfg(feature = "english-porter")]
mod english_porter;

#[cfg(feature = "english-porter")]
pub fn english_porter(env: &mut SnowballEnv) -> bool {
    return english_porter::stem(env);
}


// EnglishPorter2

#[cfg(feature = "english-porter-2")]
mod english_porter_2;

#[cfg(feature = "english-porter-2")]
pub fn english_porter_2(env: &mut SnowballEnv) -> bool {
    return english_porter_2::stem(env);
}


// EstonianFreienthal

#[cfg(feature = "estonian-freienthal")]
mod estonian_freienthal;

#[cfg(feature = "estonian-freienthal")]
pub fn estonian_freienthal(env: &mut SnowballEnv) -> bool {
    return estonian_freienthal::stem(env);
}


// Finnish

#[cfg(feature = "finnish")]
mod finnish;

#[cfg(feature = "finnish")]
pub fn finnish(env: &mut SnowballEnv) -> bool {
    return finnish::stem(env);
}


// French

#[cfg(feature = "french")]
mod french;

#[cfg(feature = "french")]
pub fn french(env: &mut SnowballEnv) -> bool {
    return french::stem(env);
}


// German

#[cfg(feature = "german")]
mod german;

#[cfg(feature = "german")]
pub fn german(env: &mut SnowballEnv) -> bool {
    return german::stem(env);
}


// Greek

#[cfg(feature = "greek")]
mod greek;

#[cfg(feature = "greek")]
pub fn greek(env: &mut SnowballEnv) -> bool {
    return greek::stem(env);
}


// HindiLightweight

#[cfg(feature = "hindi-lightweight")]
mod hindi_lightweight;

#[cfg(feature = "hindi-lightweight")]
pub fn hindi_lightweight(env: &mut SnowballEnv) -> bool {
    return hindi_lightweight::stem(env);
}


// Hungarian

#[cfg(feature = "hungarian")]
mod hungarian;

#[cfg(feature = "hungarian")]
pub fn hungarian(env: &mut SnowballEnv) -> bool {
    return hungarian::stem(env);
}


// IndonesianTala

#[cfg(feature = "indonesian-tala")]
mod indonesian_tala;

#[cfg(feature = "indonesian-tala")]
pub fn indonesian_tala(env: &mut SnowballEnv) -> bool {
    return indonesian_tala::stem(env);
}


// IrishGaelic

#[cfg(feature = "irish-gaelic")]
mod irish_gaelic;

#[cfg(feature = "irish-gaelic")]
pub fn irish_gaelic(env: &mut SnowballEnv) -> bool {
    return irish_gaelic::stem(env);
}


// Italian

#[cfg(feature = "italian")]
mod italian;

#[cfg(feature = "italian")]
pub fn italian(env: &mut SnowballEnv) -> bool {
    return italian::stem(env);
}


// LithuanianJocas

#[cfg(feature = "lithuanian-jocas")]
mod lithuanian_jocas;

#[cfg(feature = "lithuanian-jocas")]
pub fn lithuanian_jocas(env: &mut SnowballEnv) -> bool {
    return lithuanian_jocas::stem(env);
}


// Nepali

#[cfg(feature = "nepali")]
mod nepali;

#[cfg(feature = "nepali")]
pub fn nepali(env: &mut SnowballEnv) -> bool {
    return nepali::stem(env);
}


// NorwegianBokmal

#[cfg(feature = "norwegian-bokmal")]
mod norwegian_bokmal;

#[cfg(feature = "norwegian-bokmal")]
pub fn norwegian_bokmal(env: &mut SnowballEnv) -> bool {
    return norwegian_bokmal::stem(env);
}


// Portuguese

#[cfg(feature = "portuguese")]
mod portuguese;

#[cfg(feature = "portuguese")]
pub fn portuguese(env: &mut SnowballEnv) -> bool {
    return portuguese::stem(env);
}


// RomanianHeidelberg

#[cfg(feature = "romanian-heidelberg")]
mod romanian_heidelberg;

#[cfg(feature = "romanian-heidelberg")]
pub fn romanian_heidelberg(env: &mut SnowballEnv) -> bool {
    return romanian_heidelberg::stem(env);
}


// RomanianTirdea

#[cfg(feature = "romanian-tirdea")]
mod romanian_tirdea;

#[cfg(feature = "romanian-tirdea")]
pub fn romanian_tirdea(env: &mut SnowballEnv) -> bool {
    return romanian_tirdea::stem(env);
}


// Romanian

#[cfg(feature = "romanian")]
mod romanian;

#[cfg(feature = "romanian")]
pub fn romanian(env: &mut SnowballEnv) -> bool {
    return romanian::stem(env);
}


// Russian

#[cfg(feature = "russian")]
mod russian;

#[cfg(feature = "russian")]
pub fn russian(env: &mut SnowballEnv) -> bool {
    return russian::stem(env);
}


// Spanish

#[cfg(feature = "spanish")]
mod spanish;

#[cfg(feature = "spanish")]
pub fn spanish(env: &mut SnowballEnv) -> bool {
    return spanish::stem(env);
}


// Swedish

#[cfg(feature = "swedish")]
mod swedish;

#[cfg(feature = "swedish")]
pub fn swedish(env: &mut SnowballEnv) -> bool {
    return swedish::stem(env);
}


// TurkishCilden

#[cfg(feature = "turkish-cilden")]
mod turkish_cilden;

#[cfg(feature = "turkish-cilden")]
pub fn turkish_cilden(env: &mut SnowballEnv) -> bool {
    return turkish_cilden::stem(env);
}


// YiddishUrieli

#[cfg(feature = "yiddish-urieli")]
mod yiddish_urieli;

#[cfg(feature = "yiddish-urieli")]
pub fn yiddish_urieli(env: &mut SnowballEnv) -> bool {
    return yiddish_urieli::stem(env);
}

