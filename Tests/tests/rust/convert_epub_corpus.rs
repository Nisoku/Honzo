use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use crate::common::epub;
use honzo_convert::from_epub;
use honzo_core::HonzoParser;
use honzo_io::HonzoMeta;

static HZO_CACHE: LazyLock<Mutex<HashMap<&'static str, Vec<u8>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn get_hzo(file: &'static str) -> Result<Vec<u8>, String> {
    let guard = HZO_CACHE.lock().unwrap();
    if let Some(hzo) = guard.get(file) {
        return Ok(hzo.clone());
    }
    drop(guard);
    let data = epub(file);
    let hzo = from_epub(&data).map_err(|e| format!("{}: conversion error: {:?}", file, e))?;
    let mut guard = HZO_CACHE.lock().unwrap();
    guard.entry(file).or_insert_with(|| hzo.clone());
    Ok(hzo)
}

struct EpubSpec {
    file: &'static str,
    expect_cover: bool,
    expect_images: bool,
    expect_css: bool,
    min_chapters: usize,
}

const EPUBS: &[EpubSpec] = &[
    EpubSpec {
        file: "test-book.epub",
        expect_cover: false,
        expect_images: false,
        expect_css: false,
        min_chapters: 1,
    },
    EpubSpec {
        file: "Accessibility-Tests-Extended-Descriptions-v1.1.1.epub",
        expect_cover: true,
        expect_images: true,
        expect_css: true,
        min_chapters: 1,
    },
    EpubSpec {
        file: "Fundamental-Accessibility-Tests-Basic-Functionality-v2.0.0.epub",
        expect_cover: true,
        expect_images: false,
        expect_css: true,
        min_chapters: 1,
    },
    EpubSpec {
        file: "Fundamental-Accessibility-Tests-Visual-Adjustments-v2.0.0.epub",
        expect_cover: true,
        expect_images: true,
        expect_css: true,
        min_chapters: 1,
    },
    EpubSpec {
        file: "captain-charles-johnson_a-general-history-of-the-pirates.epub",
        expect_cover: true,
        expect_images: true,
        expect_css: true,
        min_chapters: 10,
    },
    EpubSpec {
        file: "captain-charles-johnson_a-general-history-of-the-pirates_advanced.epub",
        expect_cover: true,
        expect_images: false,
        expect_css: true,
        min_chapters: 10,
    },
    EpubSpec {
        file: "helen-herron-taft_recollections-of-full-years.epub",
        expect_cover: true,
        expect_images: true,
        expect_css: true,
        min_chapters: 10,
    },
    EpubSpec {
        file: "lytton-strachey_eminent-victorians.epub",
        expect_cover: true,
        expect_images: true,
        expect_css: true,
        min_chapters: 5,
    },
    EpubSpec {
        file: "lytton-strachey_eminent-victorians_advanced.epub",
        expect_cover: true,
        expect_images: false,
        expect_css: true,
        min_chapters: 5,
    },
    EpubSpec {
        file: "lytton-strachey_queen-victoria.epub",
        expect_cover: true,
        expect_images: true,
        expect_css: true,
        min_chapters: 5,
    },
    EpubSpec {
        file: "mark-twain_personal-recollections-of-joan-of-arc.epub",
        expect_cover: true,
        expect_images: true,
        expect_css: true,
        min_chapters: 10,
    },
    EpubSpec {
        file: "walter-noble-burns_tombstone.epub",
        expect_cover: true,
        expect_images: true,
        expect_css: true,
        min_chapters: 10,
    },
    EpubSpec {
        file: "walter-noble-burns_tombstone_advanced.epub",
        expect_cover: true,
        expect_images: false,
        expect_css: true,
        min_chapters: 10,
    },
];

#[test]
fn all_epubs_fully_validated() {
    let mut failures: Vec<String> = Vec::new();

    for spec in EPUBS {
        let hzo = match get_hzo(spec.file) {
            Ok(h) => h,
            Err(e) => {
                failures.push(e);
                continue;
            }
        };
        let parser = match HonzoParser::new(&hzo, 1) {
            Ok(p) => p,
            Err(e) => {
                failures.push(format!("{}: parse error: {:?}", spec.file, e));
                continue;
            }
        };

        let meta: HonzoMeta = match rmp_serde::from_slice(parser.meta_bytes().unwrap()) {
            Ok(m) => m,
            Err(e) => {
                failures.push(format!("{}: meta deserialize: {:?}", spec.file, e));
                continue;
            }
        };

        /* metadata checks */
        if meta.source_format.as_deref() != Some("epub") {
            failures.push(format!("{}: source_format should be epub", spec.file));
        }
        if meta.word_count.unwrap_or(0) == 0 {
            failures.push(format!("{}: word_count should be positive", spec.file));
        }
        if meta.reading_time_mins.unwrap_or(0) == 0 {
            failures.push(format!("{}: reading_time should be positive", spec.file));
        }

        let entries: Vec<_> = parser.toc_entries().collect();
        let chap_entries: Vec<_> = entries
            .iter()
            .filter(|e| e.chunk_type == *b"CHAP")
            .collect();

        /* chapter count */
        if chap_entries.len() < spec.min_chapters {
            failures.push(format!(
                "{}: expected >= {} CHAP chunks, got {}",
                spec.file,
                spec.min_chapters,
                chap_entries.len()
            ));
        }

        /* SIDX search index */
        let sidx = parser.find_chunk(b"SIDX");
        if sidx.is_none() {
            failures.push(format!("{}: missing SIDX chunk", spec.file));
        } else if sidx.unwrap().size_raw == 0 {
            failures.push(format!("{}: SIDX chunk is empty", spec.file));
        }

        /* CHAP type */
        for entry in &chap_entries {
            if entry.content_type_kind != 1 {
                failures.push(format!(
                    "{}: CHAP content_type_kind should be 1, got {}",
                    spec.file, entry.content_type_kind
                ));
            }
            if entry.content_type_value != 1 {
                failures.push(format!(
                    "{}: CHAP content_type_value should be 1, got {}",
                    spec.file, entry.content_type_value
                ));
            }
        }

        /* non-empty chapters with HTML markup */
        if !chap_entries.is_empty() {
            let any_has_p = chap_entries.iter().any(|entry| {
                let content = String::from_utf8_lossy(parser.chunk_bytes(entry).unwrap());
                content.contains("<p>")
            });
            if !any_has_p {
                failures.push(format!("{}: no CHAP preserves <p> tags", spec.file));
            }

            let first_content =
                String::from_utf8_lossy(parser.chunk_bytes(chap_entries[0]).unwrap());
            if !first_content.contains("</html>") {
                failures.push(format!(
                    "{}: first CHAP should contain full HTML structure",
                    spec.file
                ));
            }
            if !first_content.contains('<') {
                failures.push(format!(
                    "{}: first CHAP should contain HTML markup",
                    spec.file
                ));
            }
        }

        for entry in &chap_entries {
            let bytes = parser.chunk_bytes(entry).unwrap();
            let content = String::from_utf8_lossy(bytes);
            if !content.contains('>') {
                failures.push(format!("{}: CHAP should contain HTML tags", spec.file));
            }
            if content.trim().is_empty() {
                failures.push(format!("{}: CHAP should not be empty", spec.file));
            }
        }

        /* resources before chapters */
        let mut last_resource = 0usize;
        let mut first_chap = entries.len();
        for (i, entry) in entries.iter().enumerate() {
            match &entry.chunk_type {
                t if *t == *b"COVR"
                    || *t == *b"COVT"
                    || *t == *b"IMG_"
                    || *t == *b"CSS_"
                    || *t == *b"FONT" =>
                {
                    last_resource = i;
                }
                b"CHAP" => {
                    if i < first_chap {
                        first_chap = i;
                    }
                }
                _ => {}
            }
        }
        if last_resource > 0 && first_chap < entries.len() && last_resource >= first_chap {
            failures.push(format!(
                "{}: resources (last at idx {}) before chapters (first at idx {})",
                spec.file, last_resource, first_chap,
            ));
        }

        /* cover thumbnail */
        if spec.expect_cover {
            let covr = parser.find_chunk(b"COVR");
            if covr.is_none() {
                failures.push(format!("{}: missing COVR chunk", spec.file));
            } else if covr.unwrap().size_raw == 0 {
                failures.push(format!("{}: COVR chunk is empty", spec.file));
            }
            if let Some(covt) = parser.find_chunk(b"COVT") {
                if covt.size_raw == 0 {
                    failures.push(format!("{}: COVT chunk is empty", spec.file));
                }
            }
        }

        /* image chunks */
        if spec.expect_images {
            let img_count = entries.iter().filter(|e| e.chunk_type == *b"IMG_").count();
            if img_count == 0 {
                failures.push(format!("{}: expected IMG_ chunks, got 0", spec.file));
            }
        }

        /* CSS chunks */
        if spec.expect_css {
            let css_count = entries.iter().filter(|e| e.chunk_type == *b"CSS_").count();
            if css_count == 0 {
                failures.push(format!("{}: expected CSS_ chunks, got 0", spec.file));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "EPUB validation failures:\n  {}",
        failures.join("\n  ")
    );
}

#[test]
fn large_epub_preserves_all_chapters() {
    let hzo = get_hzo("mark-twain_personal-recollections-of-joan-of-arc.epub").unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let chap_count = parser
        .toc_entries()
        .filter(|e| e.chunk_type == *b"CHAP")
        .count();
    assert!(
        chap_count >= 30,
        "Joan of Arc: expected >= 30 chapters, got {}",
        chap_count
    );

    let total_bytes: usize = parser
        .toc_entries()
        .filter(|e| e.chunk_type == *b"CHAP")
        .map(|e| parser.chunk_bytes(&e).unwrap().len())
        .sum();

    assert!(
        total_bytes > 100_000,
        "Joan of Arc: expected > 100KB of chapter data, got {} bytes",
        total_bytes
    );
}

#[test]
fn advanced_epubs_have_same_chapters_as_standard() {
    let pairs = [
        (
            "captain-charles-johnson_a-general-history-of-the-pirates.epub",
            "captain-charles-johnson_a-general-history-of-the-pirates_advanced.epub",
        ),
        (
            "lytton-strachey_eminent-victorians.epub",
            "lytton-strachey_eminent-victorians_advanced.epub",
        ),
        (
            "walter-noble-burns_tombstone.epub",
            "walter-noble-burns_tombstone_advanced.epub",
        ),
    ];

    for (standard, advanced) in &pairs {
        let std_hzo = get_hzo(standard).unwrap();
        let adv_hzo = get_hzo(advanced).unwrap();
        let std_parser = HonzoParser::new(&std_hzo, 1).unwrap();
        let adv_parser = HonzoParser::new(&adv_hzo, 1).unwrap();

        let std_chaps = std_parser
            .toc_entries()
            .filter(|e| e.chunk_type == *b"CHAP")
            .count();
        let adv_chaps = adv_parser
            .toc_entries()
            .filter(|e| e.chunk_type == *b"CHAP")
            .count();

        assert_eq!(
            std_chaps, adv_chaps,
            "{} and {} should have same chapter count ({} vs {})",
            standard, advanced, std_chaps, adv_chaps
        );

        let std_images = std_parser
            .toc_entries()
            .filter(|e| e.chunk_type == *b"IMG_")
            .count();
        let adv_images = adv_parser
            .toc_entries()
            .filter(|e| e.chunk_type == *b"IMG_")
            .count();

        assert!(std_images > 0, "{}: standard should have images", standard);
        assert!(
            adv_images > 0,
            "{}: advanced should have image chunks (SVG)",
            advanced
        );
    }
}
