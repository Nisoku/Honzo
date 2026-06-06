/// Detect explicit pagebreaks in HTML and return (page_number, byte_offset) pairs.
/// Handles common EPUB patterns:
/// - `<span epub:type="pagebreak" id="pg42" title="42"/>`
/// - `<span class="pagebreak" id="page-42"/>`
/// - `<a id="page42" class="pagebreak"></a>`
/// - `<hr class="pagebreak"/>`
/// - `<div class="pagebreak" title="42"/>`
pub fn detect_pagebreaks(html: &str) -> Vec<(u32, u32)> {
    let mut pages: Vec<(u32, u32)> = Vec::new();
    let bytes = html.as_bytes();
    let len = html.len();
    let mut pos = 0;

    while pos < len {
        // Look for '<' followed by a tag name
        if bytes[pos] == b'<' {
            let tag_start = pos;
            let tag_end = html[pos..].find('>').map(|p| pos + p + 1).unwrap_or(len);
            let tag = &html[tag_start..tag_end];

            if is_pagebreak_tag(tag) {
                if let Some(page) = extract_page_number(tag) {
                    pages.push((page, tag_end as u32));
                }
            }

            pos = tag_end;
            continue;
        }

        pos += 1;
    }

    pages
}

fn is_pagebreak_tag(tag: &str) -> bool {
    let lower = tag.to_ascii_lowercase();

    // epub:type="pagebreak"
    if find_attr(&lower, "epub:type").as_deref() == Some("pagebreak") {
        return true;
    }

    // class="pagebreak" or class contains "pagebreak"
    if let Some(class) = find_attr(&lower, "class") {
        if class.contains("pagebreak") {
            return true;
        }
    }

    // Direct <pagebreak> tag
    if lower.starts_with("<pagebreak") || lower.starts_with("</pagebreak") {
        return true;
    }

    // role="doc-pagebreak" (EPUB 3.2)
    if find_attr(&lower, "role").as_deref() == Some("doc-pagebreak") {
        return true;
    }

    false
}

fn extract_page_number(tag: &str) -> Option<u32> {
    let lower = tag.to_ascii_lowercase();
    let mut candidates = Vec::new();

    // Try title attribute first (most common explicit page number)
    if let Some(title) = find_attr(&lower, "title") {
        candidates.push(title);
    }

    // Try id attribute (e.g. "pg42" or "page-42")
    if let Some(id) = find_attr(&lower, "id") {
        candidates.push(id);
    }

    // Try data-page attribute
    if let Some(data_page) = find_attr(&lower, "data-page") {
        candidates.push(data_page);
    }

    for candidate in candidates {
        if let Ok(n) = candidate.parse::<u32>() {
            if n > 0 && n < 1_000_000 {
                return Some(n);
            }
        }
        // Try extracting number from string like "pg42" or "page-42"
        if let Some(n) = extract_number(&candidate) {
            if n > 0 && n < 1_000_000 {
                return Some(n);
            }
        }
    }

    None
}

fn extract_number(s: &str) -> Option<u32> {
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse::<u32>().ok()
}

fn find_attr(tag_lower: &str, attr: &str) -> Option<String> {
    let attr_lower = attr.to_ascii_lowercase();
    for prefix in [&format!("{}=\"", attr_lower), &format!("{}='", attr_lower)] {
        if let Some(start) = tag_lower.find(prefix) {
            let value_start = start + prefix.len();
            let close = if prefix.ends_with('"') { '"' } else { '\'' };
            if let Some(end) = tag_lower[value_start..].find(close) {
                return Some(tag_lower[value_start..value_start + end].to_string());
            }
        }
    }
    None
}

/// Estimate page breaks for content without explicit pagebreak markers.
/// Uses a simple character-count model (~2000 chars per page for English text).
pub fn estimate_pagebreaks(
    chapter_texts: &[String],
    chapter_chunk_ids: &[u32],
    chars_per_page: u32,
) -> Vec<(u32, u32, u32)> {
    let mut entries = Vec::new();
    let mut page_num: u32 = 1;

    for (text, &chunk_id) in chapter_texts.iter().zip(chapter_chunk_ids.iter()) {
        // Strip HTML tags for character counting
        let clean = strip_html_tags(text);
        let byte_len = clean.len() as u32;

        if byte_len == 0 {
            continue;
        }

        // The first page of this chunk starts at offset 0
        entries.push((page_num, chunk_id, 0));
        page_num += 1;

        // Add additional pages based on estimated capacity
        let additional_pages = byte_len / chars_per_page;
        for i in 1..=additional_pages {
            let offset = i * chars_per_page;
            if offset < byte_len {
                entries.push((page_num, chunk_id, offset));
                page_num += 1;
            }
        }
    }

    entries
}

fn strip_html_tags(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out
}
