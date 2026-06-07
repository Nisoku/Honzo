use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct HonzoMeta {
    pub title: Option<HashMap<String, String>>,
    pub subtitle: Option<HashMap<String, String>>,
    pub description: Option<HashMap<String, String>>,
    pub original_title: Option<String>,
    pub original_lang: Option<String>,
    pub original_authors: Option<Vec<String>>,

    pub authors: Vec<String>,
    pub contributors: Option<Vec<Contributor>>,
    pub publisher: Option<String>,
    pub imprint: Option<String>,

    pub genres: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub bisac: Option<Vec<String>>,
    pub language: String,
    pub direction: Option<String>,
    pub rating: Option<String>,
    pub content_warnings: Option<Vec<String>>,

    pub series: Option<SeriesMeta>,

    pub identifiers: Option<Vec<Identifier>>,
    pub isbn_status: Option<String>,
    pub date_published: Option<String>,
    pub date_ebook_pub: Option<String>,
    pub edition: Option<String>,
    pub license: Option<String>,
    pub revisions: Option<Vec<Revision>>,

    pub layout: Option<u8>,
    pub spread_behavior: Option<String>,
    pub hints: Option<RenderHints>,

    pub word_count: Option<u32>,
    pub reading_time_mins: Option<u32>,

    pub accessibility: Option<Accessibility>,

    pub sort_title: Option<String>,
    pub sort_author: Option<String>,
    pub source_url: Option<String>,
    pub source_format: Option<String>,
    pub digitized_by: Option<String>,
    pub production_notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Contributor {
    pub name: String,
    #[serde(default)]
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SeriesMeta {
    pub title: String,
    pub position: String,
    pub arc: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Identifier {
    pub id_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Revision {
    pub version: String,
    pub date: String,
    pub note: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct RenderHints {
    pub typography: Option<String>,
    pub hyphenation: Option<bool>,
    pub widow_orphan: Option<bool>,
    pub night_mode_inversion: Option<bool>,
    pub justify: Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Accessibility {
    pub alt_text: Option<bool>,
    pub reading_order: Option<bool>,
    pub screen_reader: Option<bool>,
    pub dyslexia_friendly: Option<bool>,
    pub wcag_level: Option<String>,
}
