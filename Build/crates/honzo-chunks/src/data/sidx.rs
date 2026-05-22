use honzo_core::HonzoError;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::BTreeMap;

pub fn normalize_search_term(term: &str) -> String {
    let stemmer = Stemmer::create(Algorithm::English);
    stemmer.stem(&term.to_lowercase()).into_owned()
}

fn push_token(
    index: &mut BTreeMap<String, Vec<(u32, u32)>>,
    chunk_id: u32,
    token: &str,
    offset: usize,
) {
    if token.is_empty() {
        return;
    }
    let normalized = normalize_search_term(token);
    if !normalized.is_empty() {
        index
            .entry(normalized)
            .or_default()
            .push((chunk_id, offset as u32));
    }
}

pub fn build_sidx(chapters: &[(u32, &str)]) -> Result<Vec<u8>, HonzoError> {
    let mut index: BTreeMap<String, Vec<(u32, u32)>> = BTreeMap::new();

    for (chunk_id, text) in chapters {
        let mut token_start: Option<usize> = None;

        for (i, ch) in text.char_indices() {
            let is_delim = ch.is_whitespace() || ch.is_ascii_punctuation();
            if is_delim {
                if let Some(start) = token_start.take() {
                    let token = &text[start..i];
                    push_token(&mut index, *chunk_id, token, start);
                }
            } else if token_start.is_none() {
                token_start = Some(i);
            }
        }

        if let Some(start) = token_start.take() {
            let token = &text[start..];
            push_token(&mut index, *chunk_id, token, start);
        }
    }

    rmp_serde::to_vec(&index).map_err(|_| HonzoError::Truncated)
}
