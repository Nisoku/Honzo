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
