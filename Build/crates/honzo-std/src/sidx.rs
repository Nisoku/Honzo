use honzo_core::HonzoError;
use std::collections::BTreeMap;

pub fn build_sidx(chapters: &[(u32, &str)]) -> Result<Vec<u8>, HonzoError> {
    let mut index: BTreeMap<String, Vec<(u32, u32)>> = BTreeMap::new();

    for (chunk_id, text) in chapters {
        let mut token_start: Option<usize> = None;

        for (i, ch) in text.char_indices() {
            let is_delim = ch.is_whitespace() || ch.is_ascii_punctuation();
            if is_delim {
                if let Some(start) = token_start.take() {
                    let token = &text[start..i];
                    if !token.is_empty() {
                        let lower = token.to_ascii_lowercase();
                        let offset = start as u32;
                        index.entry(lower).or_default().push((*chunk_id, offset));
                    }
                }
            } else if token_start.is_none() {
                token_start = Some(i);
            }
        }

        if let Some(start) = token_start.take() {
            let token = &text[start..];
            if !token.is_empty() {
                let lower = token.to_ascii_lowercase();
                let offset = start as u32;
                index.entry(lower).or_default().push((*chunk_id, offset));
            }
        }
    }

    rmp_serde::to_vec(&index).map_err(|_| HonzoError::Truncated)
}
