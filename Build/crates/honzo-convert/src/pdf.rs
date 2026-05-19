//! PDF conversion using `pdf_oxide`

use crate::ConvertError;
use honzo_io::{Compression, CoverType, HonzoBuilder, MarkupType};
use pdf_oxide::PdfDocument;

pub fn convert_pdf(bytes: &[u8]) -> Result<Vec<u8>, ConvertError> {
    // Open document from memory. `pdf_oxide` exposes constructors that take
    // an owned `Vec<u8>`, so convert the slice first.
    let doc = PdfDocument::from_bytes(bytes.to_vec())
        .map_err(|e| ConvertError::IoError(e.to_string()))?;
    let page_count = doc
        .page_count()
        .map_err(|e| ConvertError::IoError(e.to_string()))?;
    if page_count == 0 {
        return Err(ConvertError::MissingSpine);
    }

    let mut builder = HonzoBuilder::new().set_layout(honzo_io::LayoutMode::Reflowable);

    for i in 0..page_count {
        // Extract page text; fall back to empty page marker
        let text = doc
            .extract_text(i)
            .map_err(|e| ConvertError::IoError(e.to_string()))?;

        let page_text = if text.trim().is_empty() {
            "\n".to_string()
        } else {
            text
        };

        builder = builder.add_chunk(
            *b"CHAP",
            page_text.as_bytes(),
            Compression::None,
            MarkupType::Html,
            CoverType::Front,
            None,
            None,
            None,
        );

        // Attempt to extract images for the page and add them as IMG_ chunks
        if let Ok(images) = doc.extract_images(i) {
            for img in images.into_iter() {
                // Prefer PNG conversion; skip image on conversion errors.
                if let Ok(buf) = img.to_png_bytes() {
                    let tag = *b"IMG_";
                    builder = builder.add_chunk(
                        tag,
                        &buf,
                        Compression::None,
                        MarkupType::Hmd,
                        CoverType::Front,
                        None,
                        None,
                        None,
                    );
                }
            }
        }
    }

    let meta = honzo_io::HonzoMeta {
        source_format: Some("pdf".to_string()),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).map_err(|e| ConvertError::IoError(e.to_string()))?;
    builder = builder.set_meta(&meta_bytes);

    builder.finalize().map_err(Into::into)
}
