use honzo_chunks::extra::{anno, drm, is_known_namespace, parse_known, sync, KnownExtra};

#[test]
fn registry_recognizes_official_namespaces() {
    assert!(is_known_namespace(anno::NAMESPACE));
    assert!(is_known_namespace(drm::NAMESPACE));
    assert!(is_known_namespace(sync::NAMESPACE));
    assert!(!is_known_namespace("com.example.custom"));
}

#[test]
fn parse_known_routes_to_typed_payloads() {
    let annotations = vec![anno::Annotation {
        chunk_id: 3,
        offset: 10,
        length: 4,
        r#type: "highlight".to_string(),
        note: Some("note".to_string()),
        color: Some("yellow".to_string()),
    }];
    let anno_body = anno::build_anno(&annotations).unwrap();

    let parsed = parse_known(anno::NAMESPACE, &anno_body)
        .expect("known namespace")
        .unwrap();

    match parsed {
        KnownExtra::Anno(got) => assert_eq!(got, annotations),
        other => panic!("expected anno payload, got {other:?}"),
    }

    let sync_body = sync::build_sync(&[sync::SyncCue {
        chunk_id: 1,
        offset: 2,
        timestamp_ms: 3,
    }])
    .unwrap();

    let sync_parsed = parse_known(sync::NAMESPACE, &sync_body)
        .expect("known namespace")
        .unwrap();

    match sync_parsed {
        KnownExtra::Sync(got) => {
            assert_eq!(got.len(), 1);
            assert_eq!(got[0].chunk_id, 1);
            assert_eq!(got[0].offset, 2);
            assert_eq!(got[0].timestamp_ms, 3);
        }
        other => panic!("expected sync payload, got {other:?}"),
    }
}

#[test]
fn unknown_namespaces_are_skipped() {
    assert!(parse_known("com.example.custom", b"ignored").is_none());
}
