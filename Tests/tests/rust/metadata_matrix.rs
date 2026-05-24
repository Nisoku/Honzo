use crate::common::fixture;
use honzo_io::{build_sidx, compute_reading_time, find_extra, new_uuid, HonzoMeta};

#[test]
fn parses_novel_metadata_and_translation_metadata() {
    let novel = fixture("novel.hzo");
    let translated = fixture("translated.hzo");

    let novel_meta: HonzoMeta = rmp_serde::from_slice(
        honzo_core::HonzoParser::new(&novel, 1)
            .unwrap()
            .meta_bytes()
            .unwrap(),
    )
    .unwrap();
    let translated_meta: HonzoMeta = rmp_serde::from_slice(
        honzo_core::HonzoParser::new(&translated, 1)
            .unwrap()
            .meta_bytes()
            .unwrap(),
    )
    .unwrap();

    assert_eq!(novel_meta.language, "en");
    assert!(novel_meta.title.as_ref().unwrap().contains_key("en"));
    assert_eq!(translated_meta.original_lang.as_deref(), Some("de"));
    assert_eq!(
        translated_meta.original_title.as_deref(),
        Some("Der Kleine Hobbit")
    );
}

#[test]
fn computes_reading_time_and_uuid_shape() {
    let uuid = new_uuid();

    assert_eq!(compute_reading_time(0), 1);
    assert_eq!(compute_reading_time(238), 1);
    assert_eq!(compute_reading_time(239), 2);
    assert_eq!(uuid.len(), 36);
    assert_eq!(uuid.chars().filter(|c| *c == '-').count(), 4);
}

#[test]
fn builds_search_index_from_chapters() {
    let chapters = [(0u32, "hello world"), (1u32, "hello rust")];
    let index = build_sidx(&chapters, "en").unwrap();
    let map: std::collections::BTreeMap<String, Vec<(u32, u32)>> =
        rmp_serde::from_slice(&index).unwrap();

    assert!(map.contains_key("hello"));
    assert_eq!(map["hello"].len(), 2);
}

#[test]
fn finds_extra_annotations_by_namespace() {
    let data = fixture("with_anno.hzo");
    let parser = honzo_core::HonzoParser::new(&data, 1).unwrap();
    let entries = honzo_io::parse_extra(parser.extra_bytes().unwrap()).unwrap();

    assert!(find_extra(&entries, "org.nisoku.anno").is_some());
    assert!(find_extra(&entries, "com.unknown.thing").is_none());
}
