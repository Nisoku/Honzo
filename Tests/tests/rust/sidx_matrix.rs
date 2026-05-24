use honzo_chunks::data::sidx::{build_sidx, normalize_search_term};
use std::collections::BTreeMap;

#[test]
fn stemming_normalizes_inflections() {
    assert_eq!(normalize_search_term("running", "en"), "run");
    assert_eq!(normalize_search_term("dogs", "en"), "dog");
}

#[test]
fn non_english_stemming_uses_correct_language() {
    assert_eq!(normalize_search_term("laufen", "de"), "lauf");
    assert_eq!(normalize_search_term("corriendo", "es"), "corr");
    assert_eq!(normalize_search_term("couraient", "fr"), "cour");
    assert_eq!(normalize_search_term("juoksee", "fi"), "juoks"); // Finnish
}

#[test]
fn unknown_language_falls_back_to_lowercasing() {
    assert_eq!(normalize_search_term("Running", "xx"), "running");
    assert_eq!(normalize_search_term("Dogs", "zz"), "dogs");
    assert_eq!(normalize_search_term("WALKING", ""), "walking");
}

#[test]
fn non_english_build_sidx_stems_correctly() {
    let sidx = build_sidx(&[(0, "laufen laufen!")], "de").unwrap();
    let index: BTreeMap<String, Vec<(u32, u32)>> = rmp_serde::from_slice(&sidx).unwrap();
    assert!(index.contains_key("lauf"));
    assert!(!index.contains_key("laufen"));
}

#[test]
fn build_sidx_preserves_raw_text_for_unknown_language() {
    let sidx = build_sidx(&[(0, "Hello World")], "unknown").unwrap();
    let index: BTreeMap<String, Vec<(u32, u32)>> = rmp_serde::from_slice(&sidx).unwrap();
    assert!(index.contains_key("hello"));
    assert!(index.contains_key("world"));
}

#[test]
fn all_supported_languages_produce_valid_stems() {
    let pairs = &[
        ("ar", "ركض"),
        ("da", "løbende"),
        ("nl", "rennen"),
        ("en", "running"),
        ("fi", "juoksee"),
        ("fr", "courir"),
        ("de", "rennen"),
        ("el", "τρέχω"),
        ("hu", "futás"),
        ("it", "correre"),
        ("no", "løper"),
        ("pt", "correndo"),
        ("ro", "alergare"),
        ("ru", "бегать"),
        ("es", "correr"),
        ("sv", "springer"),
        ("ta", "ஓடுதல்"),
        ("tr", "koşmak"),
    ];
    for (lang, word) in pairs {
        let result = normalize_search_term(word, lang);
        assert!(
            !result.is_empty(),
            "{} stemmed to empty for lang={}",
            word,
            lang
        );
    }
}

#[test]
fn build_sidx_indexes_stems() {
    let sidx = build_sidx(&[(1, "Running dogs run quickly")], "en").unwrap();
    let index: BTreeMap<String, Vec<(u32, u32)>> = rmp_serde::from_slice(&sidx).unwrap();

    assert!(index.contains_key("run"));
    assert!(index.contains_key("dog"));
    assert!(!index.contains_key("running"));
    assert!(!index.contains_key("dogs"));
}

#[test]
fn token_offsets_are_recorded() {
    let sidx = build_sidx(&[(0, "Hello, world!")], "en").unwrap();
    let index: BTreeMap<String, Vec<(u32, u32)>> = rmp_serde::from_slice(&sidx).unwrap();

    // 'hello' should be at offset 0 and 'world' at offset 7
    let hello_bucket = index.get("hello").expect("hello key");
    assert!(hello_bucket
        .iter()
        .any(|(chunk, off)| *chunk == 0 && *off == 0));

    let world_bucket = index.get("world").expect("world key");
    assert!(world_bucket
        .iter()
        .any(|(chunk, off)| *chunk == 0 && *off == 7));
}
