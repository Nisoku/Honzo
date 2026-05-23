use honzo_core::HonzoError;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::BTreeMap;

pub fn normalize_search_term(term: &str, lang: &str) -> String {
    let stemmer = match lang {
        "ar" => Stemmer::create(Algorithm::Arabic),
        "da" => Stemmer::create(Algorithm::Danish),
        "nl" => Stemmer::create(Algorithm::Dutch),
        "en" => Stemmer::create(Algorithm::English),
        "fi" => Stemmer::create(Algorithm::Finnish),
        "fr" => Stemmer::create(Algorithm::French),
        "de" => Stemmer::create(Algorithm::German),
        "el" => Stemmer::create(Algorithm::Greek),
        "hu" => Stemmer::create(Algorithm::Hungarian),
        "it" => Stemmer::create(Algorithm::Italian),
        "no" => Stemmer::create(Algorithm::Norwegian),
        "pt" => Stemmer::create(Algorithm::Portuguese),
        "ro" => Stemmer::create(Algorithm::Romanian),
        "ru" => Stemmer::create(Algorithm::Russian),
        "es" => Stemmer::create(Algorithm::Spanish),
        "sv" => Stemmer::create(Algorithm::Swedish),
        "ta" => Stemmer::create(Algorithm::Tamil),
        "tr" => Stemmer::create(Algorithm::Turkish),
        _ => return term.to_lowercase(),
    };
    stemmer.stem(&term.to_lowercase()).into_owned()
}

fn push_token(
    index: &mut BTreeMap<String, Vec<(u32, u32)>>,
    chunk_id: u32,
    token: &str,
    offset: usize,
    lang: &str,
) {
    if token.is_empty() {
        return;
    }
    let normalized = normalize_search_term(token, lang);
    if !normalized.is_empty() {
        index
            .entry(normalized)
            .or_default()
            .push((chunk_id, offset as u32));
    }
}

pub fn build_sidx(chapters: &[(u32, &str)], lang: &str) -> Result<Vec<u8>, HonzoError> {
    let mut index: BTreeMap<String, Vec<(u32, u32)>> = BTreeMap::new();

    for (chunk_id, text) in chapters {
        let mut token_start: Option<usize> = None;

        for (i, ch) in text.char_indices() {
            let is_delim = ch.is_whitespace() || ch.is_ascii_punctuation();
            if is_delim {
                if let Some(start) = token_start.take() {
                    let token = &text[start..i];
                    push_token(&mut index, *chunk_id, token, start, lang);
                }
            } else if token_start.is_none() {
                token_start = Some(i);
            }
        }

        if let Some(start) = token_start.take() {
            let token = &text[start..];
            push_token(&mut index, *chunk_id, token, start, lang);
        }
    }

    rmp_serde::to_vec(&index).map_err(|_| HonzoError::Truncated)
}
