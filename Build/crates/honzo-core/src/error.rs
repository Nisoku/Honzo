#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HonzoError {
    InvalidMagic,
    ReaderVersionTooOld {
        required: u16,
        have: u16,
    },
    BufferTooShort,
    InvalidChunkType,
    CrcMismatch {
        chunk_id: u32,
        expected: u32,
        got: u32,
    },
    EncryptedChunk {
        chunk_id: u32,
    },
    UnknownCompression(u8),
    UnknownLayoutMode(u8),
    UnknownCoverType(u8),
    UnknownMarkupType(u8),
    UnknownMathType(u8),
    InvalidMathML,
    UnknownFontEmbedding(u8),
    UnknownExtraNamespace(&'static str),
    Truncated,
}
