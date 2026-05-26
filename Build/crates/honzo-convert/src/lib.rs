use std::collections::HashMap;

use bytes::Bytes;
use lexepub::LexEpub;

use honzo_io::{
    build_sidx, compute_reading_time, generate_covt, new_uuid, Compression, CoverType,
    HonzoBuilder, HonzoMeta, Identifier, LayoutMode, MarkupType,
};
mod mobi;
mod pdf;

fn resolve_against_base(relative: &str, base_dir: &str) -> String {
    if let Some(stripped) = relative.strip_prefix('/') {
        stripped.to_string()
    } else {
        let base = std::path::Path::new(base_dir);
        let parent = base.parent().unwrap_or_else(|| std::path::Path::new(""));
        let joined = parent.join(relative);
        let mut components: Vec<&str> = Vec::new();
        for c in joined.components() {
            match c {
                std::path::Component::Normal(p) => {
                    components.push(p.to_str().unwrap_or_default());
                }
                std::path::Component::ParentDir => {
                    components.pop();
                }
                _ => {}
            }
        }
        components.join("/")
    }
}

fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let lower = tag.to_ascii_lowercase();
    let attr_lower = attr.to_ascii_lowercase();
    let patterns = [format!("{}=\"", attr_lower), format!("{}='", attr_lower)];
    for pat in &patterns {
        if let Some(start) = lower.find(pat) {
            let value_start = start + pat.len();
            let quote_char = if pat.ends_with('"') { '"' } else { '\'' };
            let end = tag[value_start..].find(quote_char)?;
            return Some(tag[value_start..value_start + end].to_string());
        }
    }
    None
}

fn normalize_path(p: &str) -> String {
    p.to_ascii_lowercase()
}

fn rewrite_html_to_ref(html: &str, chapter_path: &str, img_map: &HashMap<String, u32>) -> String {
    let mut result = String::with_capacity(html.len());
    let mut pos = 0;
    let bytes = html.as_bytes();

    while pos < html.len() {
        if pos + 4 <= html.len() && bytes[pos..pos + 4].eq_ignore_ascii_case(b"<img") {
            let tag_start = pos;
            let tag_end = html[pos..]
                .find('>')
                .map(|p| pos + p + 1)
                .unwrap_or(html.len());
            let tag = &html[tag_start..tag_end];

            if let Some(raw_src) = extract_attr(tag, "src") {
                let resolved = resolve_against_base(&raw_src, chapter_path);
                if let Some(&chunk_id) = img_map.get(&normalize_path(&resolved)) {
                    let alt = extract_attr(tag, "alt");
                    let ref_tag = if let Some(ref alt_text) = alt {
                        format!(
                            "<ref type=\"image\" chunk=\"{}\" alt=\"{}\"/>",
                            chunk_id,
                            alt_text.replace('"', "&quot;")
                        )
                    } else {
                        format!("<ref type=\"image\" chunk=\"{}\"/>", chunk_id)
                    };
                    result.push_str(&ref_tag);
                    pos = tag_end;
                    continue;
                }
            }
            result.push_str(tag);
            pos = tag_end;
            continue;
        }

        let c = html[pos..].chars().next().unwrap_or('\0');
        result.push(c);
        pos += c.len_utf8();
    }

    result
}

fn rewrite_links_to_ref(
    html: &str,
    chapter_path: &str,
    spine_map: &HashMap<String, u32>,
) -> String {
    let mut result = String::with_capacity(html.len());
    let mut pos = 0;
    let bytes = html.as_bytes();

    while pos < html.len() {
        if pos + 2 <= html.len() && bytes[pos..pos + 2].eq_ignore_ascii_case(b"<a") {
            let after_a = bytes.get(pos + 2).copied().unwrap_or(b'>');
            if after_a == b'>' || after_a == b'/' || after_a.is_ascii_whitespace() {
                let tag_start = pos;
                let tag_end = html[pos..]
                    .find('>')
                    .map(|p| pos + p + 1)
                    .unwrap_or(html.len());
                let open_tag = &html[tag_start..tag_end];

                if let Some(href) = extract_attr(open_tag, "href") {
                    let is_external = href.starts_with("http://")
                        || href.starts_with("https://")
                        || href.starts_with("mailto:");
                    let is_fragment = href.starts_with('#');

                    if !is_external && !is_fragment && !href.is_empty() && href != "#" {
                        let (path, anchor) = href.split_once('#').unwrap_or((&href, ""));
                        let resolved = resolve_against_base(path, chapter_path);
                        if let Some(&chunk_id) = spine_map.get(&normalize_path(&resolved)) {
                            let close_tag = html[tag_end..]
                                .to_ascii_lowercase()
                                .find("</a>")
                                .map(|p| tag_end + p + 4)
                                .unwrap_or(html.len());

                            if anchor.is_empty() {
                                result.push_str(&format!(
                                    "<ref type=\"chapter\" chunk=\"{}\"/>",
                                    chunk_id
                                ));
                            } else {
                                result.push_str(&format!(
                                    "<ref type=\"chapter\" chunk=\"{}\" anchor=\"{}\"/>",
                                    chunk_id, anchor
                                ));
                            }
                            pos = close_tag;
                            continue;
                        }
                    }
                }
                result.push_str(open_tag);
                pos = tag_end;
                continue;
            }
        }

        let c = html[pos..].chars().next().unwrap_or('\0');
        result.push(c);
        pos += c.len_utf8();
    }

    result
}

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
        .set_language(&language)
        .set_auto_sidx(false);

    let mut next_chunk_id: u32 = 0;

    // Extract image alt texts from parsed ASTs
    let mut img_alt_map: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut parsed_chapters: Vec<lexepub::ParsedChapter> = Vec::new();
    let parser = lexepub::ChapterParser::new().with_both();
    for chap_path in spine.iter() {
        if let Ok(ch_data) = epub.read_resource(chap_path).await {
            let chapter = lexepub::Chapter::new(chap_path.clone(), String::new(), ch_data);
            if let Ok(parsed) = parser.parse_chapter(chapter) {
                parsed_chapters.push(parsed);
            }
        }
    }
    if !parsed_chapters.is_empty() {
        img_alt_map = honzo_chunks::data::img::collect_and_resolve_img_alts_async(
            &parsed_chapters,
            &mut epub,
        )
        .await;
    }

    let mut img_path_to_chunk: HashMap<String, u32> = HashMap::new();

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
                img_path_to_chunk.insert(normalize_path(&path), next_chunk_id);
                next_chunk_id += 1;
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
                    next_chunk_id += 1;
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
                let alt_text = img_alt_map
                    .get(&path)
                    .or_else(|| img_alt_map.get(&item.href))
                    .and_then(|a| if a.is_empty() { None } else { Some(a.as_str()) })
                    .or_else(|| {
                        // Basename fallback
                        let fname = std::path::Path::new(&path)
                            .file_name()
                            .and_then(|s| s.to_str())?;
                        img_alt_map
                            .iter()
                            .find(|(k, v)| {
                                std::path::Path::new(k).file_name().and_then(|s| s.to_str())
                                    == Some(fname)
                                    && !v.is_empty()
                            })
                            .map(|(_, v)| v.as_str())
                    });

                img_path_to_chunk.insert(normalize_path(&path), next_chunk_id);
                builder = builder.add_chunk(
                    *b"IMG_",
                    &data,
                    Compression::None,
                    MarkupType::Markdown,
                    CoverType::Front,
                    alt_text,
                    None,
                    None,
                );
                next_chunk_id += 1;
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
                next_chunk_id += 1;
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
                next_chunk_id += 1;
            }
        }
    }

    let mut chapter_texts: Vec<String> = Vec::new();
    let mut chapter_chunk_ids: Vec<u32> = Vec::new();

    // Build spine path -> chunk ID map for cross-chapter link rewriting
    let mut spine_path_to_chunk: HashMap<String, u32> = HashMap::new();
    let mut chap_id = next_chunk_id;
    for path in &spine {
        let item = manifest.iter().find(|m| resolve(&m.href) == *path);
        let Some(item) = item else { continue };
        if is_html_type(&item.media_type) {
            spine_path_to_chunk.insert(normalize_path(path), chap_id);
            chap_id += 1;
        }
    }

    // Extract text only for valid HTML chapters
    for path in &spine {
        let item = manifest.iter().find(|m| resolve(&m.href) == *path);
        let Some(item) = item else { continue };
        if is_html_type(&item.media_type) {
            let Ok(html_bytes) = epub.read_resource(path).await else {
                continue;
            };
            let text = String::from_utf8_lossy(&html_bytes).to_string();
            chapter_texts.push(text);
        }
    }

    for path in &spine {
        let item = manifest.iter().find(|m| resolve(&m.href) == *path);
        let Some(item) = item else { continue };
        if !is_html_type(&item.media_type) {
            continue;
        }

        let Ok(html_bytes) = epub.read_resource(path).await else {
            continue;
        };

        let html_text = String::from_utf8_lossy(&html_bytes).to_string();
        let rewritten = if !img_path_to_chunk.is_empty() {
            rewrite_html_to_ref(&html_text, path, &img_path_to_chunk)
        } else {
            html_text
        };
        let rewritten = if !spine_path_to_chunk.is_empty() {
            rewrite_links_to_ref(&rewritten, path, &spine_path_to_chunk)
        } else {
            rewritten
        };

        let chap_title = chap_titles.get(path).map(|s| s.as_str());
        chapter_chunk_ids.push(next_chunk_id);

        builder = builder.add_chunk(
            *b"CHAP",
            rewritten.as_bytes(),
            Compression::None,
            MarkupType::Html,
            CoverType::Front,
            chap_title,
            None,
            None,
        );
        next_chunk_id += 1;
    }

    if chapter_chunk_ids.len() != chapter_texts.len() {
        return Err(ConvertError::IoError(format!(
            "chapter text count mismatch: {} text entries for {} chapter chunks",
            chapter_texts.len(),
            chapter_chunk_ids.len()
        )));
    }

    let sidx_refs: Vec<(u32, &str)> = chapter_chunk_ids
        .iter()
        .zip(chapter_texts.iter())
        .map(|(chunk_id, text)| (*chunk_id, text.as_str()))
        .collect();

    if !sidx_refs.is_empty() {
        let sidx = build_sidx(&sidx_refs, &language)?;
        builder = builder.add_chunk(
            *b"SIDX",
            &sidx,
            Compression::Lz4,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        );
        builder = builder.set_flags(0x20);
    }

    let mut title_map = HashMap::new();
    if let Some(ref t) = title {
        title_map.insert(language.clone(), t.clone());
    }

    let word_count: u32 = chapter_texts
        .iter()
        .map(|t| t.split_whitespace().count() as u32)
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
