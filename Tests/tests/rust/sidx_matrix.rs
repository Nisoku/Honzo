use honzo_chunks::data::sidx::{build_sidx, normalize_search_term};
use std::collections::BTreeMap;

#[test]
fn stemming_normalizes_inflections() {
    assert_eq!(normalize_search_term("running"), "run");
    assert_eq!(normalize_search_term("dogs"), "dog");
}

#[test]
fn build_sidx_indexes_stems() {
    let sidx = build_sidx(&[(1, "Running dogs run quickly")]).unwrap();
    let index: BTreeMap<String, Vec<(u32, u32)>> = rmp_serde::from_slice(&sidx).unwrap();

    assert!(index.contains_key("run"));
    assert!(index.contains_key("dog"));
    assert!(!index.contains_key("running"));
    assert!(!index.contains_key("dogs"));
}

#[test]
fn token_offsets_are_recorded() {
    let sidx = build_sidx(&[(0, "Hello, world!")]).unwrap();
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
