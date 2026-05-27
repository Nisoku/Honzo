use std::collections::HashMap;

pub fn resolve_against_base(relative: &str, base_dir: &str) -> String {
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

pub fn extract_attr(tag: &str, attr: &str) -> Option<String> {
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

pub fn normalize_path(p: &str) -> String {
    p.to_ascii_lowercase()
}

pub fn rewrite_html_to_ref(
    html: &str,
    chapter_path: &str,
    img_map: &HashMap<String, u32>,
) -> String {
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

pub fn rewrite_links_to_ref(
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
