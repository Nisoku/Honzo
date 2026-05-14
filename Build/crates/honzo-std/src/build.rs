use honzo_core::{
    Compression, CoverType, FontEmbedding, HonzoBuilder, HonzoError, LayoutMode, MarkupType,
    PmapEntry,
};

pub struct Builder {
    inner: HonzoBuilder,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            inner: HonzoBuilder::new(),
        }
    }

    pub fn set_layout(mut self, layout: LayoutMode) -> Self {
        self.inner = self.inner.set_layout(layout);
        self
    }

    pub fn set_flags(mut self, flags: u32) -> Self {
        self.inner = self.inner.set_flags(flags);
        self
    }

    pub fn add_chunk(
        mut self,
        tag: [u8; 4],
        data: &[u8],
        compression: Compression,
        markup_type: MarkupType,
        cover_type: CoverType,
        alt_text: Option<&str>,
        font_embedding: Option<FontEmbedding>,
        font_license_url: Option<&str>,
    ) -> Self {
        self.inner = self.inner.add_chunk(
            tag,
            data,
            compression,
            markup_type,
            cover_type,
            alt_text,
            font_embedding,
            font_license_url,
        );
        self
    }

    pub fn add_pmap_entry(mut self, entry: PmapEntry) -> Self {
        self.inner = self.inner.add_pmap_entry(entry);
        self
    }

    pub fn set_meta(mut self, msgpack: &[u8]) -> Self {
        self.inner = self.inner.set_meta(msgpack);
        self
    }

    pub fn set_extra(mut self, extra: &[u8]) -> Self {
        self.inner = self.inner.set_extra(extra);
        self
    }

    pub fn finalize(self) -> Result<Vec<u8>, HonzoError> {
        self.inner.finalize()
    }
}
