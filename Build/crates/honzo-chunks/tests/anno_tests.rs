#![cfg(test)]

use honzo_chunks::extra::anno::*;

#[test]
fn test_annotation_creation() {
    let annotation = Annotation {
        chunk_id: 1,
        offset: 100,
        length: 20,
        r#type: "highlight".to_string(),
        note: Some("important text".to_string()),
        color: Some("#ffff00".to_string()),
    };
    
    assert_eq!(annotation.chunk_id, 1);
    assert_eq!(annotation.offset, 100);
    assert_eq!(annotation.length, 20);
    assert_eq!(annotation.r#type, "highlight");
    assert_eq!(annotation.note, Some("important text".to_string()));
    assert_eq!(annotation.color, Some("#ffff00".to_string()));
}

#[test]
fn test_annotation_with_optional_fields() {
    let annotation = Annotation {
        chunk_id: 1,
        offset: 100,
        length: 20,
        r#type: "highlight".to_string(),
        note: None,
        color: None,
    };
    
    assert_eq!(annotation.chunk_id, 1);
    assert_eq!(annotation.offset, 100);
    assert_eq!(annotation.length, 20);
    assert_eq!(annotation.r#type, "highlight");
    assert!(annotation.note.is_none());
    assert!(annotation.color.is_none());
}