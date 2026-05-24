use std::collections::HashMap;

use bytes::Bytes;
use lexepub::LexEpub;

use honzo_io::{
    compute_reading_time, generate_covt, new_uuid, Compression, CoverType, HonzoBuilder,
    HonzoMeta, Identifier, LayoutMode, MarkupType,
};
mod mobi;
mod pdf;

#[derive(Debug)]
pub enum ConvertError {
    UnsupportedFormat,
    MissingSpine,
    MissingMetadata,
    IoError(String),
    HonzoError(honzo_io::HonzoError),
}

impl From<honzo_io::HonzoError> for ConvertError {
    fn from(e: honzo_io::HonzoError) -> Self {
        ConvertError::HonzoError(e)
    }
}

impl From<lexepub::LexEpubError> for ConvertError {
    fn from(e: lexepub::LexEpubError) -> Self {
        ConvertError::IoError(e.to_string())
    }
}

pub fn from_epub(bytes: &[u8]) -> Result<Vec<u8>, ConvertError> {
    futures::executor::block_on(convert_epub(Bytes::copy_from_slice(bytes)))
}

pub fn from_mobi(_bytes: &[u8]) -> Result<Vec<u8>, ConvertError> {
    mobi::convert_mobi(_bytes)
}

pub fn from_pdf(_bytes: &[u8]) -> Result<Vec<u8>, ConvertError> {
    pdf::convert_pdf(_bytes)
}

async fn convert_epub(data: Bytes) -> Result<Vec<u8>, ConvertError> {
    let mut epub = LexEpub::from_bytes(data).await?;

    let meta = epub.get_metadata().await?;
    let toc = epub.get_toc().await.unwrap_or_default();

    let title = meta.title.clone();
    let creators = meta.authors.clone();
    let language = meta
        .languages
        .first()
        .cloned()
        .unwrap_or_else(|| "en".to_string());

    let mut chap_titles: HashMap<String, String> = HashMap::new();
    for entry in &toc {
        chap_titles.insert(entry.chapter_href.clone(), entry.title.clone());
    }

    let container_xml = epub
        .read_resource("META-INF/container.xml")
        .await
        .map_err(|_| ConvertError::MissingMetadata)?;
    let container_str = String::from_utf8_lossy(&container_xml);
    let opf_path = parse_container(&container_str)?;

    let opf_xml = epub
        .read_resource(&opf_path)
        .await
        .map_err(|_| ConvertError::MissingMetadata)?;
    let opf_str = String::from_utf8_lossy(&opf_xml).to_string();
    let opf = parse_opf(&opf_str);

    let opf_dir = opf_path
        .rfind('/')
        .map(|i| opf_path[..=i].to_string())
        .unwrap_or_default();
    let resolve = |href: &str| -> String {
        href.strip_prefix('/')
            .map_or_else(|| format!("{}{}", opf_dir, href), ToString::to_string)
    };

    let manifest: Vec<ManifestItem> = opf
        .manifest
        .into_iter()
        .map(|(id, href, mt, props)| ManifestItem {
            id,
            href,
            media_type: mt,
            properties: props,
        })
        .collect();

    let spine: Vec<String> = opf
        .spine
        .iter()
        .filter_map(|idref| {
            manifest
                .iter()
                .find(|m| m.id == *idref)
                .map(|m| resolve(&m.href))
        })
        .collect();

    if spine.is_empty() {
        return Err(ConvertError::MissingSpine);
    }

    let mut builder = HonzoBuilder::new()
        .set_layout(LayoutMode::Reflowable)
        .set_language(&language);

    // Cover image
    if let Some(ref cid) = opf.cover_id {
        if let Some(item) = manifest.iter().find(|m| m.id == *cid) {
            let path = resolve(&item.href);
            if let Ok(covr_data) = epub.read_resource(&path).await {
                builder = builder.add_chunk(
                    *b"COVR",
                    &covr_data,
                    Compression::None,
                    MarkupType::Markdown,
                    CoverType::Front,
                    Some(&path),
                    None,
                    None,
                );
                if let Ok(covt) = generate_covt(&covr_data) {
                    builder = builder.add_chunk(
                        *b"COVT",
                        &covt,
                        Compression::None,
                        MarkupType::Markdown,
                        CoverType::Front,
                        None,
                        None,
                        None,
                    );
                }
            }
        }
    }

    for item in &manifest {
        if item.media_type.starts_with("image/")
            && Some(item.id.as_str()) != opf.cover_id.as_deref()
        {
            let path = resolve(&item.href);
            if let Ok(data) = epub.read_resource(&path).await {
                builder = builder.add_chunk(
                    *b"IMG_",
                    &data,
                    Compression::None,
                    MarkupType::Markdown,
                    CoverType::Front,
                    Some(&path),
                    None,
                    None,
                );
            }
        }
    }

    for item in &manifest {
        if item.media_type == "text/css" {
            let path = resolve(&item.href);
            if let Ok(data) = epub.read_resource(&path).await {
                builder = builder.add_chunk(
                    *b"CSS_",
                    &data,
                    Compression::None,
                    MarkupType::Markdown,
                    CoverType::Front,
                    None,
                    None,
                    None,
                );
            }
        }
    }

    for item in &manifest {
        let is_font = item.media_type.starts_with("font/")
            || item.media_type == "application/x-font-ttf"
            || item.media_type == "application/x-font-opentype"
            || item.media_type == "application/font-woff"
            || item.media_type == "application/font-woff2";
        if is_font {
            let path = resolve(&item.href);
            if let Ok(data) = epub.read_resource(&path).await {
                builder = builder.add_chunk(
                    *b"FONT",
                    &data,
                    Compression::None,
                    MarkupType::Markdown,
                    CoverType::Front,
                    None,
                    Some(honzo_io::FontEmbedding::Allowed),
                    None,
                );
            }
        }
    }

    let mut chap_texts: Vec<(u32, String)> = Vec::new();

    for (chunk_id, path) in spine.iter().enumerate() {
        let item = manifest.iter().find(|m| resolve(&m.href) == *path);
        let Some(item) = item else { continue };
        if !is_html_type(&item.media_type) {
            continue;
        }

        let Ok(html_bytes) = epub.read_resource(path).await else {
            continue;
        };
        let html_str = String::from_utf8_lossy(&html_bytes);
        let text = extract_text_for_index(&html_str);

        if text.is_empty() {
            continue;
        }

        let chap_title = chap_titles.get(path).map(|s| s.as_str());
        chap_texts.push((chunk_id as u32, text));

        builder = builder.add_chunk(
            *b"CHAP",
            &html_bytes,
            Compression::None,
            MarkupType::Html,
            CoverType::Front,
            chap_title,
            None,
            None,
        );
    }

    let mut title_map = HashMap::new();
    if let Some(ref t) = title {
        title_map.insert(language.clone(), t.clone());
    }

    let word_count: u32 = chap_texts
        .iter()
        .map(|(_, t)| t.split_whitespace().count() as u32)
        .sum();

    let honzo_meta = HonzoMeta {
        title: if title_map.is_empty() {
            None
        } else {
            Some(title_map)
        },
        authors: creators,
        language: language.clone(),
        identifiers: Some(vec![Identifier {
            id_type: "uuid".to_string(),
            value: new_uuid(),
        }]),
        source_format: Some("epub".to_string()),
        word_count: Some(word_count),
        reading_time_mins: Some(compute_reading_time(word_count)),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&honzo_meta).unwrap();
    builder = builder.set_meta(&meta_bytes);

    builder.finalize().map_err(Into::into)
}

/// Extract text from HTML for search indexing only.
/// Chunks store raw HTML; this is used solely for building the SIDX search index.
#[cfg(not(feature = "lowmem"))]
fn extract_text_for_index(html: &str) -> String {
    let dom = match tl::parse(html, tl::ParserOptions::default()) {
        Ok(d) => d,
        Err(_) => return String::new(),
    };
    let parser = dom.parser();
    let mut out = String::new();
    for &handle in dom.children() {
        extract_text_node(handle, parser, &mut out);
    }
    out.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(not(feature = "lowmem"))]
fn extract_text_node(handle: tl::NodeHandle, parser: &tl::Parser, output: &mut String) {
    use html_escape::decode_html_entities;
    if let Some(node) = handle.get(parser) {
        match node {
            tl::Node::Raw(text_bytes) => {
                let s = text_bytes.as_utf8_str().to_string();
                let decoded = decode_html_entities(&s);
                output.push_str(&decoded);
            }
            tl::Node::Tag(tag) => {
                let tag_name = tag.name().as_utf8_str();
                let is_block = matches!(
                    tag_name.as_ref(),
                    "p" | "div"
                        | "h1"
                        | "h2"
                        | "h3"
                        | "h4"
                        | "h5"
                        | "h6"
                        | "br"
                        | "li"
                        | "tr"
                        | "td"
                        | "th"
                        | "blockquote"
                );
                for child_handle in tag.children().top().iter() {
                    extract_text_node(*child_handle, parser, output);
                }
                if is_block {
                    output.push('\n');
                }
            }
            _ => {}
        }
    }
}

#[cfg(feature = "lowmem")]
fn extract_text_for_index(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    let mut tag_buf = String::new();
    let mut last_was_space = false;

    for c in html.chars() {
        if in_tag {
            if c == '>' {
                in_tag = false;
                let tag = tag_buf.trim().trim_start_matches('/').to_ascii_lowercase();
                if tag.starts_with('p')
                    || tag.starts_with("div")
                    || tag.starts_with("br")
                    || tag.starts_with('h')
                    || tag.starts_with("li")
                {
                    out.push('\n');
                }
                tag_buf.clear();
            } else {
                tag_buf.push(c);
            }
        } else if c == '<' {
            in_tag = true;
            tag_buf.clear();
        } else {
            if c.is_whitespace() {
                if !last_was_space {
                    out.push(' ');
                    last_was_space = true;
                }
            } else {
                out.push(c);
                last_was_space = false;
            }
        }
    }

    out.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[allow(dead_code)]
struct ManifestItem {
    id: String,
    href: String,
    media_type: String,
    properties: String,
}

fn is_html_type(media_type: &str) -> bool {
    matches!(
        media_type,
        "application/xhtml+xml" | "text/html" | "application/html"
    )
}

fn parse_container(xml: &str) -> Result<String, ConvertError> {
    let mut reader = quick_xml::Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(ref e) | quick_xml::events::Event::Empty(ref e))
                if e.name().as_ref() == b"rootfile" =>
            {
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"full-path" {
                        return Ok(String::from_utf8_lossy(&attr.value).to_string());
                    }
                }
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => return Err(ConvertError::IoError(format!("XML error: {}", e))),
            _ => {}
        }
        buf.clear();
    }
    Err(ConvertError::MissingMetadata)
}

#[allow(dead_code)]
struct OpfData {
    title: Option<String>,
    creators: Vec<String>,
    language: Option<String>,
    cover_id: Option<String>,
    manifest: Vec<(String, String, String, String)>,
    spine: Vec<String>,
}

fn parse_opf(xml: &str) -> OpfData {
    let mut reader = quick_xml::Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let mut title = None;
    let mut creators = Vec::new();
    let mut language = None;
    let mut cover_id = None;
    let mut manifest = Vec::new();
    let mut spine = Vec::new();
    let mut in_metadata = false;
    let mut in_manifest = false;
    let mut in_spine = false;
    let mut current_tag = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(ref e))
            | Ok(quick_xml::events::Event::Empty(ref e)) => {
                let tag = e.name().as_ref().to_vec();
                current_tag = tag.clone();
                match tag.as_slice() {
                    b"metadata" => in_metadata = true,
                    b"manifest" => in_manifest = true,
                    b"spine" => in_spine = true,
                    b"item" if in_manifest => {
                        let mut id = String::new();
                        let mut href = String::new();
                        let mut mt = String::new();
                        let mut props = String::new();
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"id" => id = String::from_utf8_lossy(&attr.value).to_string(),
                                b"href" => href = String::from_utf8_lossy(&attr.value).to_string(),
                                b"media-type" => {
                                    mt = String::from_utf8_lossy(&attr.value).to_string()
                                }
                                b"properties" => {
                                    props = String::from_utf8_lossy(&attr.value).to_string()
                                }
                                _ => {}
                            }
                        }
                        if props.contains("cover-image") && cover_id.is_none() {
                            cover_id = Some(id.clone());
                        }
                        if !id.is_empty() && !href.is_empty() {
                            manifest.push((id, href, mt, props));
                        }
                    }
                    b"itemref" if in_spine => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"idref" {
                                spine.push(String::from_utf8_lossy(&attr.value).to_string());
                            }
                        }
                    }
                    b"meta" if in_metadata => {
                        let mut name = String::new();
                        let mut content = String::new();
                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"name" => name = String::from_utf8_lossy(&attr.value).to_string(),
                                b"content" => {
                                    content = String::from_utf8_lossy(&attr.value).to_string()
                                }
                                _ => {}
                            }
                        }
                        if name == "cover" && !content.is_empty() && cover_id.is_none() {
                            cover_id = Some(content);
                        }
                    }
                    _ => {}
                }
            }
            Ok(quick_xml::events::Event::Text(ref e)) => {
                let text = e.decode().unwrap_or_default().to_string();
                if text.trim().is_empty() || !in_metadata {
                    buf.clear();
                    continue;
                }
                match current_tag.as_slice() {
                    b"title" | b"dc:title" => title.get_or_insert(text),
                    b"creator" | b"dc:creator" => {
                        creators.push(text);
                        continue;
                    }
                    b"language" | b"dc:language" => language.get_or_insert(text),
                    _ => {
                        buf.clear();
                        continue;
                    }
                };
            }
            Ok(quick_xml::events::Event::End(ref e)) => {
                let tag = e.name().as_ref().to_vec();
                match tag.as_slice() {
                    b"metadata" => in_metadata = false,
                    b"manifest" => in_manifest = false,
                    b"spine" => in_spine = false,
                    _ => {}
                }
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    OpfData {
        title,
        creators,
        language,
        cover_id,
        manifest,
        spine,
    }
}
