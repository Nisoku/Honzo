use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;

use honzo_io::*;

use crate::refs::normalize_path;
use crate::ConvertError;

#[derive(serde::Deserialize)]
struct MdProjectConfig {
    title: Option<String>,
    subtitle: Option<String>,
    description: Option<String>,
    author: Option<String>,
    authors: Option<Vec<String>>,
    language: Option<String>,
    cover: Option<String>,
}

pub fn from_markdown_file(path: &Path) -> Result<Vec<u8>, ConvertError> {
    let content = fs::read_to_string(path)
        .map_err(|e| ConvertError::IoError(format!("Cannot read {:?}: {}", path, e)))?;
    let project_dir = path.parent().unwrap_or_else(|| Path::new("."));

    let (config, body) = extract_frontmatter(&content);
    let chapters = split_chapters(body);
    if chapters.is_empty() {
        return Err(ConvertError::MdParseError(
            "No content found in markdown file".into(),
        ));
    }

    build_honzo(project_dir, config, &chapters)
}

pub fn from_markdown_dir(path: &Path) -> Result<Vec<u8>, ConvertError> {
    if !path.is_dir() {
        return Err(ConvertError::IoError(format!(
            "{:?} is not a directory",
            path
        )));
    }

    let config = read_dir_config(path);

    let mut md_files: Vec<_> = fs::read_dir(path)
        .map_err(|e| ConvertError::IoError(e.to_string()))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown"))
        })
        .collect();
    md_files.sort_by_key(|e| e.file_name());

    if md_files.is_empty() {
        return Err(ConvertError::MdParseError(format!(
            "No .md files found in {:?}",
            path
        )));
    }

    let mut chapters: Vec<(Option<String>, String)> = Vec::new();
    for entry in &md_files {
        let content =
            fs::read_to_string(entry.path()).map_err(|e| ConvertError::IoError(e.to_string()))?;
        let (file_config, body) = extract_frontmatter(&content);
        let file_chapters = split_chapters(body);

        if !file_chapters.is_empty() {
            for (i, (title, chap_content)) in file_chapters.into_iter().enumerate() {
                let effective_title = title.or_else(|| {
                    if i == 0 {
                        file_config
                            .as_ref()
                            .and_then(|c| c.title.clone())
                            .or_else(|| {
                                entry
                                    .path()
                                    .file_stem()
                                    .map(|s| s.to_string_lossy().to_string())
                            })
                    } else {
                        None
                    }
                });
                chapters.push((effective_title, chap_content));
            }
        } else if !body.trim().is_empty() {
            let title = file_config
                .as_ref()
                .and_then(|c| c.title.clone())
                .or_else(|| {
                    entry
                        .path()
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                });
            chapters.push((title, body.to_string()));
        }
    }

    build_honzo(path, config, &chapters)
}

fn safe_project_path(project_dir: &Path, rel: &str) -> Option<PathBuf> {
    if rel.is_empty() {
        return None;
    }
    let rel_path = Path::new(rel);
    if rel_path.is_absolute() {
        return None;
    }
    let mut out = project_dir.to_path_buf();
    for component in rel_path.components() {
        match component {
            std::path::Component::Normal(_) => out.push(component),
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir
            | std::path::Component::RootDir
            | std::path::Component::Prefix(_) => return None,
        }
    }
    Some(out)
}

fn extract_frontmatter(content: &str) -> (Option<MdProjectConfig>, &str) {
    let content = content.trim_start();
    let rest = content.strip_prefix("---").unwrap_or(content);
    if rest.len() == content.len() {
        return (None, content);
    }

    let after_open = rest.trim_start_matches('\n');
    if let Some(end) = after_open.find("\n---") {
        let yaml_str = &after_open[..end];
        let body = &after_open[end + 4..];
        if let Ok(config) = serde_yaml::from_str::<MdProjectConfig>(yaml_str) {
            return (Some(config), body);
        }
    }

    (None, content)
}

fn read_dir_config(dir: &Path) -> Option<MdProjectConfig> {
    let json_path = dir.join("honzo.json");
    fs::read_to_string(json_path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
}

fn split_chapters(body: &str) -> Vec<(Option<String>, String)> {
    let body = body.trim();
    if body.is_empty() {
        return Vec::new();
    }

    let has_h1 = Regex::new(r"(?m)^#\s+(.+)")
        .expect("invalid heading regex")
        .is_match(body);
    if has_h1 {
        split_by_headings(body)
    } else {
        split_by_rules(body)
    }
}

fn split_by_headings(body: &str) -> Vec<(Option<String>, String)> {
    let re = Regex::new(r"(?m)^#\s+(.+)$").expect("invalid heading regex");
    let mut chapters = Vec::new();
    let mut last_title: Option<String> = None;
    let mut last_start = 0usize;
    let mut seen_first = false;

    for cap in re.captures_iter(body) {
        let m = cap.get(0).expect("capture group 0 missing");
        if seen_first {
            let section = body[last_start..m.start()].trim().to_string();
            if !section.is_empty() {
                chapters.push((last_title.take(), section));
            }
        } else {
            let preamble = body[last_start..m.start()].trim().to_string();
            if !preamble.is_empty() {
                chapters.push((None, preamble));
            }
        }
        seen_first = true;
        last_title = Some(cap[1].to_string());
        last_start = m.start();
    }

    if seen_first && last_start < body.len() {
        let section = body[last_start..].trim().to_string();
        if !section.is_empty() {
            chapters.push((last_title, section));
        }
    }

    chapters
}

fn split_by_rules(body: &str) -> Vec<(Option<String>, String)> {
    let re = Regex::new(r"(?m)^-{3,}\s*$").expect("invalid rule regex");
    let mut chapters = Vec::new();
    let mut last_end = 0usize;

    for m in re.find_iter(body) {
        let section = body[last_end..m.start()].trim().to_string();
        if !section.is_empty() {
            chapters.push((None, section));
        }
        last_end = m.end();
    }

    let section = body[last_end..].trim().to_string();
    if !section.is_empty() {
        chapters.push((None, section));
    }

    chapters
}

fn extract_image_refs(md: &str) -> Vec<String> {
    let re = Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").expect("invalid image regex");
    re.captures_iter(md).map(|c| c[2].to_string()).collect()
}

fn build_honzo(
    project_dir: &Path,
    config: Option<MdProjectConfig>,
    chapters: &[(Option<String>, String)],
) -> Result<Vec<u8>, ConvertError> {
    let lang = config
        .as_ref()
        .and_then(|c| c.language.as_deref())
        .unwrap_or("en");
    let mut builder = HonzoBuilder::new().set_auto_sidx(true).set_language(lang);

    let mut img_path_to_chunk: HashMap<String, u32> = HashMap::new();
    let mut chunk_id: u32 = 0;

    // Cover image
    if let Some(cover_path) = config.as_ref().and_then(|c| c.cover.as_ref()) {
        if let Some(cover_full) = safe_project_path(project_dir, cover_path) {
            if let Ok(data) = fs::read(&cover_full) {
                builder = builder.add_chunk(
                    *b"COVR",
                    &data,
                    Compression::None,
                    MarkupType::Markdown,
                    CoverType::Front,
                    None,
                    None,
                    None,
                );
                img_path_to_chunk.insert(normalize_path(cover_path), chunk_id);
                chunk_id += 1;

                if let Ok(covt) = generate_covt(&data) {
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
                    chunk_id += 1;
                }
            }
        }
    }

    // Collect all unique image paths referenced across all chapters
    let mut all_img_paths: Vec<String> = Vec::new();
    for (_, chap_content) in chapters {
        for p in extract_image_refs(chap_content) {
            let normalized = normalize_path(&p);
            if !all_img_paths.contains(&normalized) {
                all_img_paths.push(normalized);
            }
        }
    }

    // Add image chunks
    for img_path in &all_img_paths {
        if let Some(img_full) = safe_project_path(project_dir, img_path) {
            if let Ok(data) = fs::read(&img_full) {
                img_path_to_chunk.insert(normalize_path(img_path), chunk_id);
                builder = builder.add_chunk(
                    *b"IMG_",
                    &data,
                    Compression::None,
                    MarkupType::Markdown,
                    CoverType::Front,
                    None,
                    None,
                    None,
                );
                chunk_id += 1;
            }
        }
    }

    // Rewrite markdown image refs and add chapter chunks
    for (title, chap_content) in chapters {
        let rewritten = rewrite_md_image_refs(chap_content, &img_path_to_chunk);
        builder = builder.add_chunk(
            *b"CHAP",
            rewritten.as_bytes(),
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            title.as_deref(),
            None,
            None,
        );
    }

    // Build metadata
    let title_map = config.as_ref().and_then(|c| {
        c.title.clone().map(|t| {
            let mut m = HashMap::new();
            let l = c.language.clone().unwrap_or_else(|| "en".to_string());
            m.insert(l, t);
            m
        })
    });

    let mut authors: Vec<String> = config
        .as_ref()
        .and_then(|c| c.authors.clone())
        .unwrap_or_default();
    if authors.is_empty() {
        if let Some(ref a) = config.as_ref().and_then(|c| c.author.clone()) {
            authors.push(a.clone());
        }
    }

    let word_count: u32 = chapters
        .iter()
        .map(|(_, c)| c.split_whitespace().count() as u32)
        .sum();

    let meta = HonzoMeta {
        title: title_map,
        subtitle: config.as_ref().and_then(|c| {
            c.subtitle.clone().map(|s| {
                let mut m = HashMap::new();
                let l = c.language.clone().unwrap_or_else(|| "en".to_string());
                m.insert(l, s);
                m
            })
        }),
        description: config.as_ref().and_then(|c| {
            c.description.clone().map(|d| {
                let mut m = HashMap::new();
                let l = c.language.clone().unwrap_or_else(|| "en".to_string());
                m.insert(l, d);
                m
            })
        }),
        authors,
        language: lang.to_string(),
        identifiers: Some(vec![Identifier {
            id_type: "uuid".to_string(),
            value: new_uuid(),
        }]),
        source_format: Some("markdown".to_string()),
        word_count: Some(word_count),
        reading_time_mins: Some(compute_reading_time(word_count)),
        ..Default::default()
    };
    let meta_bytes = rmp_serde::to_vec(&meta).map_err(|e| ConvertError::IoError(e.to_string()))?;
    builder = builder.set_meta(&meta_bytes);

    builder.finalize().map_err(ConvertError::HonzoError)
}

/// Escape a string for safe inclusion inside a double-quoted XML attribute value.
fn xml_escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

fn rewrite_md_image_refs(md: &str, img_map: &HashMap<String, u32>) -> String {
    let re = Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").expect("invalid image regex");
    re.replace_all(md, |caps: &regex::Captures| {
        let alt = &caps[1];
        let path = &caps[2];
        let normalized = normalize_path(path);
        if let Some(&chunk_id) = img_map.get(&normalized) {
            if alt.is_empty() {
                format!("<ref type=\"image\" chunk=\"{}\"/>", chunk_id)
            } else {
                format!(
                    "<ref type=\"image\" chunk=\"{}\" alt=\"{}\"/>",
                    chunk_id,
                    xml_escape(alt)
                )
            }
        } else {
            caps[0].to_string()
        }
    })
    .to_string()
}
