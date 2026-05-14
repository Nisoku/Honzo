use std::collections::HashMap;
use std::fs;
use std::path::Path;

use honzo_std::{
    build_sidx, compute_reading_time, generate_covt, new_uuid, Builder, Compression, CoverType,
    HonzoMeta, Identifier, LayoutMode, MarkupType, PmapEntry, SeriesMeta,
};

fn fixtures_dir() -> &'static Path {
    Path::new("../Tests/fixtures")
}

fn corpus_dir() -> &'static Path {
    Path::new("../Tests/corpus")
}

fn minimal_meta() -> HonzoMeta {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Minimal".to_string());
    HonzoMeta {
        title: Some(title),
        authors: vec!["Test".to_string()],
        language: "en".to_string(),
        ..Default::default()
    }
}

fn gen_minimal() {
    let meta = rmp_serde::to_vec(&minimal_meta()).unwrap();
    let hzo = Builder::new().set_meta(&meta).finalize().unwrap();
    fs::write(fixtures_dir().join("minimal.hzo"), &hzo).unwrap();
}

fn lorem() -> &'static str {
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
}

fn gen_novel() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Test Novel".to_string());
    let mut desc = HashMap::new();
    desc.insert(
        "en".to_string(),
        "A test novel for the Honzo format.".to_string(),
    );

    let meta = HonzoMeta {
        title: Some(title),
        description: Some(desc),
        authors: vec!["Test Author".to_string()],
        language: "en".to_string(),
        genres: Some(vec!["Fiction".to_string()]),
        series: Some(SeriesMeta {
            title: "Test Series".to_string(),
            position: "1/3".to_string(),
            arc: None,
        }),
        identifiers: Some(vec![Identifier {
            id_type: "uuid".to_string(),
            value: new_uuid(),
        }]),
        word_count: Some(150),
        reading_time_mins: Some(compute_reading_time(150)),
        ..Default::default()
    };

    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();
    let covr = make_cover_jpeg();

    let hzo = Builder::new()
        .set_layout(LayoutMode::Reflowable)
        .add_chunk(
            *b"COVR",
            &covr,
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CSS_",
            b"body { color: black; }",
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    fs::write(fixtures_dir().join("novel.hzo"), &hzo).unwrap();
}

fn make_cover_jpeg() -> Vec<u8> {
    let mut buf = Vec::new();
    let img = image::ImageBuffer::from_fn(200, 300, |_, _| image::Rgb([200u8, 150u8, 100u8]));
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 75);
    encoder
        .encode(&img, 200, 300, image::ExtendedColorType::Rgb8)
        .unwrap();
    buf
}

fn make_dummy_img(w: u32, h: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    let img = image::ImageBuffer::from_fn(w, h, |x, y| {
        image::Rgb([(x * 255 / w) as u8, (y * 255 / h) as u8, 128u8])
    });
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 75);
    encoder
        .encode(&img, w, h, image::ExtendedColorType::Rgb8)
        .unwrap();
    buf
}

fn gen_manga() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Test Manga".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Mangaka".to_string()],
        language: "ja".to_string(),
        direction: Some("rtl".to_string()),
        spread_behavior: Some("double_right".to_string()),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let covr = make_cover_jpeg();
    let mut b = Builder::new().set_layout(LayoutMode::Scroll).add_chunk(
        *b"COVR",
        &covr,
        Compression::None,
        MarkupType::Hmd,
        CoverType::Front,
        None,
        None,
        None,
    );

    let img = make_dummy_img(400, 600);
    for _ in 0..5 {
        b = b.add_chunk(
            *b"IMG_",
            &img,
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        );
    }

    let hzo = b.set_meta(&meta_bytes).finalize().unwrap();
    fs::write(fixtures_dir().join("manga.hzo"), &hzo).unwrap();
}

fn gen_textbook() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Test Textbook".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Professor".to_string()],
        language: "en".to_string(),
        layout: Some(1),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let hzo = Builder::new()
        .set_layout(LayoutMode::Fixed)
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Html,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Html,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"MATH",
            br"<math><mi>x</mi><mo>+</mo><mn>1</mn></math>",
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_pmap_entry(PmapEntry {
            print_page: 1,
            chunk_id: 0,
            byte_offset: 0,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 2,
            chunk_id: 0,
            byte_offset: 50,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 3,
            chunk_id: 1,
            byte_offset: 0,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 4,
            chunk_id: 1,
            byte_offset: 30,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 5,
            chunk_id: 2,
            byte_offset: 0,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 6,
            chunk_id: 2,
            byte_offset: 10,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 7,
            chunk_id: 2,
            byte_offset: 25,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 8,
            chunk_id: 2,
            byte_offset: 40,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 9,
            chunk_id: 2,
            byte_offset: 55,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 10,
            chunk_id: 2,
            byte_offset: 70,
        })
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    fs::write(fixtures_dir().join("textbook.hzo"), &hzo).unwrap();
}

fn gen_multilang() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Multilingual".to_string());
    title.insert("ja".to_string(), "多言語".to_string());
    title.insert("ar".to_string(), "متعدد اللغات".to_string());

    let mut desc = HashMap::new();
    desc.insert("en".to_string(), "English description".to_string());
    desc.insert("ja".to_string(), "日本語の説明".to_string());
    desc.insert("ar".to_string(), "وصف باللغة العربية".to_string());

    let meta = HonzoMeta {
        title: Some(title),
        description: Some(desc),
        authors: vec!["Polyglot".to_string()],
        language: "en".to_string(),
        direction: Some("ltr".to_string()),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let hzo = Builder::new()
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    fs::write(fixtures_dir().join("multilang.hzo"), &hzo).unwrap();
}

fn gen_with_sidx() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "With SIDX".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Author".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let chapters = [
        (0u32, "The quick brown fox jumps over the lazy dog."),
        (1u32, "Pack my box with five dozen liquor jugs."),
        (2u32, "How vexingly quick daft zebras jump!"),
    ];
    let sidx = build_sidx(&chapters).unwrap();

    let hzo = Builder::new()
        .set_flags(0x20)
        .add_chunk(
            *b"CHAP",
            chapters[0].1.as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            chapters[1].1.as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            chapters[2].1.as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"SIDX",
            &sidx,
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    fs::write(fixtures_dir().join("with_sidx.hzo"), &hzo).unwrap();
}

#[derive(serde::Serialize)]
struct Annotation {
    chunk_id: u32,
    offset: u32,
    length: u32,
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
}

fn gen_with_anno() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "With Annotations".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Author".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let annotations = vec![
        Annotation {
            chunk_id: 0,
            offset: 10,
            length: 5,
            r#type: "highlight".to_string(),
            note: Some("Great passage!".to_string()),
            color: Some("yellow".to_string()),
        },
        Annotation {
            chunk_id: 1,
            offset: 0,
            length: 0,
            r#type: "bookmark".to_string(),
            note: None,
            color: None,
        },
        Annotation {
            chunk_id: 2,
            offset: 20,
            length: 15,
            r#type: "note".to_string(),
            note: Some("Interesting fact".to_string()),
            color: None,
        },
    ];
    let anno_body = rmp_serde::to_vec(&annotations).unwrap();

    let mut extra = Vec::new();
    extra.extend_from_slice(b"ANNO");
    extra.extend_from_slice(&(15u16).to_le_bytes());
    extra.extend_from_slice(b"org.nisoku.anno");
    extra.extend_from_slice(&(anno_body.len() as u32).to_le_bytes());
    extra.extend_from_slice(&anno_body);

    let hzo = Builder::new()
        .set_flags(0x40)
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .set_extra(&extra)
        .finalize()
        .unwrap();
    fs::write(fixtures_dir().join("with_anno.hzo"), &hzo).unwrap();
}

fn gen_with_covt() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "With Cover Thumbnail".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Author".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let covr = make_cover_jpeg();
    let covt = generate_covt(&covr).unwrap();

    let hzo = Builder::new()
        .add_chunk(
            *b"COVR",
            &covr,
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"COVT",
            &covt,
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    fs::write(fixtures_dir().join("with_covt.hzo"), &hzo).unwrap();
}

fn gen_translated() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "The Hobbit".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Translator".to_string()],
        original_title: Some("Der Kleine Hobbit".to_string()),
        original_lang: Some("de".to_string()),
        original_authors: Some(vec!["J.R.R. Tolkien".to_string()]),
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let hzo = Builder::new()
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    fs::write(fixtures_dir().join("translated.hzo"), &hzo).unwrap();
}

fn gen_series() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Series Book 2".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Series Author".to_string()],
        language: "en".to_string(),
        series: Some(SeriesMeta {
            title: "Test Series".to_string(),
            position: "2/5".to_string(),
            arc: None,
        }),
        identifiers: Some(vec![Identifier {
            id_type: "isbn".to_string(),
            value: "978-3-16-148410-0".to_string(),
        }]),
        isbn_status: Some("assigned".to_string()),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let hzo = Builder::new()
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    fs::write(fixtures_dir().join("series.hzo"), &hzo).unwrap();
}

fn make_dummy_font() -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(b"OTTO");
    data.extend_from_slice(&[0u8; 8]);
    data
}

fn gen_with_fonts() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "With Fonts".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Designer".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let font_data = make_dummy_font();

    let hzo = Builder::new()
        .add_chunk(
            *b"FONT",
            &font_data,
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            Some(honzo_std::FontEmbedding::Allowed),
            Some("https://example.com/font-license"),
        )
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    fs::write(fixtures_dir().join("with_fonts.hzo"), &hzo).unwrap();
}

fn gen_max_chunks() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Max Chunks".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Author".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let mut b = Builder::new();
    for i in 0..1000 {
        let text = format!("Chapter {}.\n", i);
        b = b.add_chunk(
            *b"CHAP",
            text.as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        );
    }
    let hzo = b.set_meta(&meta_bytes).finalize().unwrap();
    fs::write(fixtures_dir().join("max_chunks.hzo"), &hzo).unwrap();
}

fn gen_compressed(path: &str, compression: Compression) {
    let mut title = HashMap::new();
    title.insert("en".to_string(), path.to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Author".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let covr = make_cover_jpeg();
    let text = lorem().repeat(10);

    let hzo = Builder::new()
        .add_chunk(
            *b"COVR",
            &covr,
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            text.as_bytes(),
            compression,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            text.as_bytes(),
            compression,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            text.as_bytes(),
            compression,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    fs::write(fixtures_dir().join(path), &hzo).unwrap();
}

// --- Corpus files ---

fn gen_bad_magic() {
    let mut out = Vec::new();
    out.extend_from_slice(b"NOTHONO..............");
    out.resize(60, 0);
    fs::write(corpus_dir().join("bad_magic.hzo"), &out).unwrap();
}

fn gen_version_too_new() {
    let mut out = Vec::new();
    out.extend_from_slice(b"HONO");
    out.push(2);
    out.push(0);
    out.extend_from_slice(&65535u16.to_le_bytes());
    out.extend_from_slice(&[0u8; 44]);
    fs::write(corpus_dir().join("version_too_new.hzo"), &out).unwrap();
}

fn gen_truncated_head() {
    fs::write(corpus_dir().join("truncated_head.hzo"), b"HONO\x01").unwrap();
}

fn gen_truncated_toc() {
    let mut out = Vec::new();
    out.extend_from_slice(b"HONO");
    out.push(1);
    out.push(0);
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&5u32.to_le_bytes());
    out.extend_from_slice(&100u64.to_le_bytes());
    out.extend_from_slice(&0u64.to_le_bytes());
    out.extend_from_slice(&0u64.to_le_bytes());
    out.extend_from_slice(&0u64.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    fs::write(corpus_dir().join("truncated_toc.hzo"), &out).unwrap();
}

fn gen_truncated_data() {
    let mut out = Vec::new();
    out.extend_from_slice(b"HONO");
    out.push(1);
    out.push(0);
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&1u32.to_le_bytes());
    out.extend_from_slice(&100u64.to_le_bytes());
    out.extend_from_slice(&500u64.to_le_bytes());
    out.extend_from_slice(&0u64.to_le_bytes());
    out.extend_from_slice(&0u64.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());

    out.extend_from_slice(&1u32.to_le_bytes());
    out.extend_from_slice(b"CHAP");
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&0u64.to_le_bytes());
    out.extend_from_slice(&100u32.to_le_bytes());
    out.extend_from_slice(&100u32.to_le_bytes());
    out.extend_from_slice(&[0u8; 4]);
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());

    fs::write(corpus_dir().join("truncated_data.hzo"), &out).unwrap();
}

fn gen_crc_mismatch() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "CRC Mismatch".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Author".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let mut hzo = Builder::new()
        .add_chunk(
            *b"CHAP",
            b"Some content",
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();

    let toc_size = u64::from_le_bytes(hzo[24..32].try_into().unwrap()) as usize;
    let data_start = 4 + 48 + toc_size;
    if data_start < hzo.len() {
        hzo[data_start] ^= 0xFF;
    }
    fs::write(corpus_dir().join("crc_mismatch.hzo"), &hzo).unwrap();
}

fn gen_unknown_chunk_type() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Unknown Chunk".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Author".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let hzo = Builder::new()
        .add_chunk(
            *b"XXXX",
            b"unknown data",
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    fs::write(corpus_dir().join("unknown_chunk_type.hzo"), &hzo).unwrap();
}

fn gen_unknown_extra_ns() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Unknown Extra".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Author".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let mut extra = Vec::new();
    extra.extend_from_slice(b"TEST");
    extra.extend_from_slice(&(17u16).to_le_bytes());
    extra.extend_from_slice(b"com.unknown.thing");
    extra.extend_from_slice(&5u32.to_le_bytes());
    extra.extend_from_slice(b"hello");

    let hzo = Builder::new()
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .set_extra(&extra)
        .finalize()
        .unwrap();
    fs::write(corpus_dir().join("unknown_extra_ns.hzo"), &hzo).unwrap();
}

fn gen_empty_extra() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Empty Extra".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Author".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let hzo = Builder::new()
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .set_extra(b"")
        .finalize()
        .unwrap();
    fs::write(corpus_dir().join("empty_extra.hzo"), &hzo).unwrap();
}

fn gen_empty_meta() {
    let hzo = Builder::new()
        .add_chunk(
            *b"CHAP",
            lorem().as_bytes(),
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(b"")
        .finalize()
        .unwrap();
    fs::write(corpus_dir().join("empty_meta.hzo"), &hzo).unwrap();
}

fn gen_zero_chunks() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Zero Chunks".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Author".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let hzo = Builder::new().set_meta(&meta_bytes).finalize().unwrap();
    fs::write(corpus_dir().join("zero_chunks.hzo"), &hzo).unwrap();
}

fn gen_encrypted_chunk() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Encrypted".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Author".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let mut hzo = Builder::new()
        .add_chunk(
            *b"CHAP",
            b"secret data",
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();

    const FLAGS_OFFSET: usize = 56 + 4 + 4 + 8 + 4 + 4 + 1 + 1 + 1;
    if hzo.len() > FLAGS_OFFSET {
        hzo[FLAGS_OFFSET] |= 0x01;
    }
    fs::write(corpus_dir().join("encrypted_chunk.hzo"), &hzo).unwrap();
}

fn gen_large_alt_text() {
    let mut title = HashMap::new();
    title.insert("en".to_string(), "Large Alt Text".to_string());
    let meta = HonzoMeta {
        title: Some(title),
        authors: vec!["Author".to_string()],
        language: "en".to_string(),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();

    let alt = "x".repeat(500);
    let img = make_dummy_img(100, 100);
    let hzo = Builder::new()
        .add_chunk(
            *b"IMG_",
            &img,
            Compression::None,
            MarkupType::Hmd,
            CoverType::Front,
            Some(&alt),
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    fs::write(corpus_dir().join("large_alt_text.hzo"), &hzo).unwrap();
}

fn main() {
    let dirs = [fixtures_dir(), corpus_dir()];
    for d in &dirs {
        fs::create_dir_all(d).unwrap();
    }

    gen_minimal();
    println!("Generated minimal.hzo");

    gen_novel();
    println!("Generated novel.hzo");

    gen_manga();
    println!("Generated manga.hzo");

    gen_textbook();
    println!("Generated textbook.hzo");

    gen_multilang();
    println!("Generated multilang.hzo");

    gen_with_sidx();
    println!("Generated with_sidx.hzo");

    gen_with_anno();
    println!("Generated with_anno.hzo");

    gen_with_covt();
    println!("Generated with_covt.hzo");

    gen_translated();
    println!("Generated translated.hzo");

    gen_series();
    println!("Generated series.hzo");

    gen_with_fonts();
    println!("Generated with_fonts.hzo");

    gen_max_chunks();
    println!("Generated max_chunks.hzo");

    gen_compressed("compressed_zlib.hzo", Compression::Zlib);
    println!("Generated compressed_zlib.hzo");

    gen_compressed("compressed_zstd.hzo", Compression::Zstd);
    println!("Generated compressed_zstd.hzo");

    gen_bad_magic();
    println!("Generated bad_magic.hzo");

    gen_version_too_new();
    println!("Generated version_too_new.hzo");

    gen_truncated_head();
    println!("Generated truncated_head.hzo");

    gen_truncated_toc();
    println!("Generated truncated_toc.hzo");

    gen_truncated_data();
    println!("Generated truncated_data.hzo");

    gen_crc_mismatch();
    println!("Generated crc_mismatch.hzo");

    gen_unknown_chunk_type();
    println!("Generated unknown_chunk_type.hzo");

    gen_unknown_extra_ns();
    println!("Generated unknown_extra_ns.hzo");

    gen_empty_extra();
    println!("Generated empty_extra.hzo");

    gen_empty_meta();
    println!("Generated empty_meta.hzo");

    gen_zero_chunks();
    println!("Generated zero_chunks.hzo");

    gen_encrypted_chunk();
    println!("Generated encrypted_chunk.hzo");

    gen_large_alt_text();
    println!("Generated large_alt_text.hzo");

    println!("All fixtures generated successfully!");
}
