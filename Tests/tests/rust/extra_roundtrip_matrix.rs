use crate::common::fixture;
use honzo_chunks::extra::{anno, drm, sync};
use honzo_core::{Compression, CoverType, HonzoParser, MarkupType};
use honzo_io::{HonzoBuilder, HonzoMeta, HonzoStream};

#[test]
fn anno_roundtrip_through_parser() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let annotations = vec![
        anno::Annotation {
            chunk_id: 0,
            offset: 10,
            length: 20,
            r#type: "highlight".to_string(),
            note: Some("important passage".to_string()),
            color: Some("yellow".to_string()),
        },
        anno::Annotation {
            chunk_id: 0,
            offset: 100,
            length: 15,
            r#type: "underline".to_string(),
            note: None,
            color: None,
        },
    ];

    let file = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            b"chapter one content here",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_annotation(&annotations)
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();

    let parser = HonzoParser::new(&file, 1).unwrap();
    let extra = parser.extra_bytes().unwrap();
    let entries = honzo_io::parse_extra(extra).unwrap();
    let entry = honzo_io::find_extra(&entries, honzo_chunks::extra::ANNO_NAMESPACE).unwrap();
    let got = honzo_chunks::extra::anno::parse_anno(&entry.body).unwrap();
    assert_eq!(got, annotations);
}

#[test]
fn anno_roundtrip_through_stream() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let annotations = vec![anno::Annotation {
        chunk_id: 0,
        offset: 5,
        length: 8,
        r#type: "comment".to_string(),
        note: Some("hello".to_string()),
        color: Some("red".to_string()),
    }];

    let file = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            b"some text",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_annotation(&annotations)
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();

    let mut stream = HonzoStream::open(std::io::Cursor::new(&file), 1).unwrap();
    let got = stream.annotations().unwrap();
    assert_eq!(got, annotations);
}

#[test]
fn sync_cues_roundtrip_through_parser() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let cues = vec![
        sync::new_audio_cue(0, 100, 5000),
        sync::new_video_cue(0, 500, 10000),
        sync::new_page_cue(0, 0, 42),
    ];

    let file = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            b"sync test content",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_sync_cue(&cues)
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();

    let parser = HonzoParser::new(&file, 1).unwrap();
    let extra = parser.extra_bytes().unwrap();
    let entries = honzo_io::parse_extra(extra).unwrap();
    let entry = honzo_io::find_extra(&entries, honzo_chunks::extra::SYNC_NAMESPACE).unwrap();
    let got = honzo_chunks::extra::sync::parse_sync(&entry.body).unwrap();
    assert_eq!(got.len(), 3);
    assert_eq!(got[0].chunk_id, 0);
    assert_eq!(got[0].offset, 100);
    assert_eq!(got[0].timestamp_ms, 5000);
    assert_eq!(got[1].timestamp_ms, 10000);
    assert_eq!(got[2].timestamp_ms, 42);
}

#[test]
fn sync_cues_roundtrip_through_stream() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let cue = sync::new_media_segment_cue(sync::SyncType::Audio, 0, 200, 3000, 1500, "media1");

    let file = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            b"stream sync test",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_sync_cue(&[cue.clone()])
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();

    let mut stream = HonzoStream::open(std::io::Cursor::new(&file), 1).unwrap();
    let got = stream.sync_cues().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0], cue);
}

#[test]
fn combined_anno_and_sync_in_one_file() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let annotations = vec![anno::Annotation {
        chunk_id: 0,
        offset: 0,
        length: 5,
        r#type: "highlight".to_string(),
        note: None,
        color: None,
    }];
    let cues = vec![sync::new_audio_cue(0, 0, 0)];

    let file = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            b"combined test",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_annotation(&annotations)
        .add_sync_cue(&cues)
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();

    let parser = HonzoParser::new(&file, 1).unwrap();
    let extra = parser.extra_bytes().unwrap();
    let entries = honzo_io::parse_extra(extra).unwrap();

    let anno_entry = honzo_io::find_extra(&entries, honzo_chunks::extra::ANNO_NAMESPACE).unwrap();
    let got_annos = honzo_chunks::extra::anno::parse_anno(&anno_entry.body).unwrap();
    assert_eq!(got_annos, annotations);

    let sync_entry = honzo_io::find_extra(&entries, honzo_chunks::extra::SYNC_NAMESPACE).unwrap();
    let got_cues = honzo_chunks::extra::sync::parse_sync(&sync_entry.body).unwrap();
    assert_eq!(got_cues.len(), 1);
}

#[test]
fn extra_entries_persist_through_parse_extra() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let custom_ns = "com.example.custom.roundtrip";
    let custom_body = b"hello extra roundtrip";

    let file = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            b"extra test",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_extra_entry(*b"XTRA", custom_ns, custom_body)
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();

    let parser = HonzoParser::new(&file, 1).unwrap();
    let entries = honzo_io::parse_extra(parser.extra_bytes().unwrap()).unwrap();
    let entry = honzo_io::find_extra(&entries, custom_ns).unwrap();
    assert_eq!(entry.body, custom_body);
}

#[test]
fn annotations_from_fixture() {
    let data = fixture("with_anno.hzo");
    let parser = honzo_core::HonzoParser::new(&data, 1).unwrap();
    let extra = parser.extra_bytes().unwrap();
    let entries = honzo_io::parse_extra(extra).unwrap();
    let entry = honzo_io::find_extra(&entries, honzo_chunks::extra::ANNO_NAMESPACE)
        .expect("fixture should have anno extra entry");
    let annotations = honzo_chunks::extra::anno::parse_anno(&entry.body).unwrap();
    assert!(!annotations.is_empty(), "fixture should have annotations");
}

#[test]
fn stream_yields_same_annotations_as_parser() {
    let data = fixture("with_anno.hzo");
    let parser = honzo_core::HonzoParser::new(&data, 1).unwrap();
    let extra = parser.extra_bytes().unwrap();
    let entries = honzo_io::parse_extra(extra).unwrap();
    let entry = honzo_io::find_extra(&entries, honzo_chunks::extra::ANNO_NAMESPACE).unwrap();
    let parser_annos = honzo_chunks::extra::anno::parse_anno(&entry.body).unwrap();

    let mut stream = HonzoStream::open(std::io::Cursor::new(&data), 1).unwrap();
    let stream_annos = stream.annotations().unwrap();

    assert_eq!(parser_annos, stream_annos);
}

#[test]
fn drm_roundtrip_parse_and_build() {
    let envelope = drm::DrmEnvelope {
        algorithm: "AES-256-CBC".to_string(),
        iv: vec![0u8; 16],
        ciphertext: vec![1u8, 2, 3, 4, 5],
    };
    let body = drm::build_drm(&envelope).unwrap();
    let parsed = drm::parse_drm(&body).unwrap();
    assert_eq!(parsed, envelope);
}

#[test]
fn drm_known_namespace_is_recognized() {
    assert!(honzo_chunks::extra::is_known_namespace(
        honzo_chunks::extra::DRM_NAMESPACE
    ));
    let known = honzo_chunks::extra::parse_known(
        honzo_chunks::extra::DRM_NAMESPACE,
        &drm::build_drm(&drm::DrmEnvelope {
            algorithm: "AES-256-CBC".to_string(),
            iv: vec![0u8; 16],
            ciphertext: vec![1u8, 2, 3],
        })
        .unwrap(),
    )
    .unwrap()
    .unwrap();
    match known {
        honzo_chunks::extra::KnownExtra::Drm(_) => {}
        _ => panic!("expected Drm wrapped"),
    }
}
