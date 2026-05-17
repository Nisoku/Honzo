//! MOBI conversion using the `mobi` crate.

use crate::ConvertError;
use honzo_std::{Compression, CoverType, HonzoBuilder, MarkupType};

pub fn convert_mobi(bytes: &[u8]) -> Result<Vec<u8>, ConvertError> {
    // Heuristic: if file is a ZIP (EPUB), delegate
    if bytes.len() >= 4 && &bytes[..4] == b"PK\x03\x04" {
        return super::from_epub(bytes);
    }

    // Try parsing MOBI via the `mobi` crate. `mobi::Mobi::new` expects an
    // owned Vec<u8> (it is generic over AsRef<Vec<u8>>), so convert the
    // incoming slice first.
    match mobi::Mobi::new(bytes.to_vec()) {
        Ok(m) => {
            // `content_as_string` returns a Result<String, _>.
            let content = match m.content_as_string() {
                Ok(s) => s,
                Err(_) => m.content_as_string_lossy(),
            };

            let mut builder = HonzoBuilder::new().set_layout(honzo_std::LayoutMode::Reflowable);

            if !content.trim().is_empty() {
                builder = builder.add_chunk(
                    *b"CHAP",
                    content.as_bytes(),
                    Compression::None,
                    MarkupType::Html,
                    CoverType::Front,
                    None,
                    None,
                    None,
                );
            }

            let meta = honzo_std::HonzoMeta {
                source_format: Some("mobi".to_string()),
                ..Default::default()
            };
            let meta_bytes =
                rmp_serde::to_vec(&meta).map_err(|e| ConvertError::IoError(e.to_string()))?;
            builder = builder.set_meta(&meta_bytes);

            builder.finalize().map_err(Into::into)
        }
        Err(_) => Err(ConvertError::UnsupportedFormat),
    }
}
