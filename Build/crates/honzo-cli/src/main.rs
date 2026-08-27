use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use clap::{Parser, Subcommand};
use honzo_io::*;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use rayon::prelude::*;

#[derive(Parser)]
#[command(name = "honzo", about = "Honzo ebook format tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print HEAD fields, TOC summary, and META fields
    Info { file: PathBuf },
    /// Full structured dump of .hzo file
    Inspect {
        file: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Extract a single chunk by chunk_id
    Extract {
        file: PathBuf,
        #[arg(long)]
        chunk: u32,
        #[arg(long)]
        out: PathBuf,
    },
    /// Extract all chunks to a directory
    ExtractAll {
        file: PathBuf,
        #[arg(long = "out-dir")]
        out_dir: PathBuf,
    },
    /// Build a .hzo file from a JSON spec
    Build {
        spec: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Convert epub/mobi/pdf to .hzo
    Convert { input: PathBuf, out: PathBuf },
    /// Batch-convert all matching files to .hzo
    ConvertBatch {
        /// Glob pattern for input files (e.g. "*.epub" or "books/**/*.epub")
        pattern: String,
        /// Output directory
        out_dir: PathBuf,
    },
    /// Parse and validate .hzo file
    Validate { file: PathBuf },
    /// Query SIDX search index
    Search {
        file: PathBuf,
        #[arg(long)]
        query: String,
    },
    /// Render .hzo as a file tree
    Tree { file: PathBuf },
}

fn read_file(path: &Path) -> io::Result<Vec<u8>> {
    let mut f = fs::File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf)
}

fn read_file_or_exit(path: &Path) -> Vec<u8> {
    read_file(path).unwrap_or_else(|e| {
        eprintln!("Error: cannot read {}: {}", path.display(), e);
        std::process::exit(1);
    })
}

struct ProgressTracker {
    pb: ProgressBar,
    stage_name: Mutex<String>,
    stage_total: AtomicU32,
    stage_done: AtomicU32,
}

impl ProgressTracker {
    fn new(pb: ProgressBar) -> Self {
        Self {
            pb,
            stage_name: Mutex::new(String::new()),
            stage_total: AtomicU32::new(0),
            stage_done: AtomicU32::new(0),
        }
    }
}

impl honzo_convert::ConvertProgress for ProgressTracker {
    fn stage(&self, name: &str) {
        *self.stage_name.lock().expect("progress lock poisoned") = name.to_string();
        self.stage_done.store(0, Ordering::Relaxed);
        self.stage_total.store(0, Ordering::Relaxed);
        self.pb.set_message(format!("{}...", name));
    }

    fn advance(&self) {
        self.stage_done.fetch_add(1, Ordering::Relaxed);
        let done = self.stage_done.load(Ordering::Relaxed);
        self.pb.set_message(format!(
            "{}... ({})",
            self.stage_name.lock().expect("progress lock poisoned"),
            done
        ));
        self.pb.inc(1);
    }
}

fn cmd_info(file: &Path) {
    let data = read_file_or_exit(file);
    let p = honzo_core::HonzoParser::new(&data, 1).unwrap_or_else(|e| {
        eprintln!("Parse error: {:?}", e);
        std::process::exit(1);
    });
    let head = p.head();

    println!("{}", file.display());
    println!("  Version: {}.{}", head.version_major, head.version_minor);
    println!("  Min reader version: {}", head.min_reader_version);
    println!("  Chunk count: {}", head.chunk_count);
    println!("  Layout: {:?}", head.layout_mode());
    if head.has_drm() {
        println!("  DRM: present");
    }
    if head.has_sidx() {
        println!("  SIDX: present");
    }
    if head.has_anno() {
        println!("  Annotations: present");
    }
    if head.has_sync() {
        println!("  Sync: present");
    }
    println!("  TOC: {} bytes", head.toc_size);
    println!("  DATA: {} bytes", head.data_size);
    println!("  EXTRA: {} bytes", head.extra_size);
    println!("  META: {} bytes", head.meta_size);

    println!("\n  Chunks:");
    for entry in p.toc_entries() {
        let tag = core::str::from_utf8(&entry.chunk_type).unwrap_or("????");
        println!(
            "    [{}] {}  off={} packed={} raw={} crc32=0x{:08x}",
            entry.chunk_id, tag, entry.offset, entry.size_compressed, entry.size_raw, entry.crc32
        );
    }

    if let Ok(meta_bytes) = p.meta_bytes() {
        if let Ok(meta) = rmp_serde::from_slice::<HonzoMeta>(meta_bytes) {
            println!("\n  Metadata:");
            if let Some(ref t) = meta.title {
                for (lang, title) in t {
                    println!("    Title ({}): {}", lang, title);
                }
            }
            for a in &meta.authors {
                println!("    Author: {}", a);
            }
            println!("    Language: {}", meta.language);
            if let Some(ref wc) = meta.word_count {
                println!("    Words: {}", wc);
            }
            if let Some(ref rt) = meta.reading_time_mins {
                println!("    Reading time: {} min", rt);
            }
        }
    }
}

fn cmd_inspect(file: &Path, json: bool) {
    let data = read_file_or_exit(file);
    let p = honzo_core::HonzoParser::new(&data, 1).unwrap_or_else(|e| {
        eprintln!("Parse error: {:?}", e);
        std::process::exit(1);
    });

    #[derive(serde::Serialize, Debug)]
    struct Dump {
        path: String,
        version_major: u8,
        version_minor: u8,
        min_reader_version: u16,
        flags: u32,
        chunk_count: u32,
        toc_size: u64,
        data_size: u64,
        extra_size: u64,
        meta_size: u64,
        layout_mode: u8,
        has_drm: bool,
        has_sidx: bool,
        has_anno: bool,
        has_sync: bool,
        toc: Vec<TocItem>,
        pmap: Vec<PmapItem>,
        meta: serde_json::Value,
        extra_entries: Vec<ExtraItem>,
    }

    #[derive(serde::Serialize, Debug)]
    struct TocItem {
        chunk_type: String,
        chunk_id: u32,
        offset: u64,
        size_compressed: u32,
        size_raw: u32,
        compression: u8,
        content_type_kind: u8,
        content_type_value: u8,
        cover_type: u8,
        flags: u8,
        crc32: u32,
        alt_text: Option<String>,
        is_encrypted: bool,
    }

    #[derive(serde::Serialize, Debug)]
    struct PmapItem {
        print_page: u32,
        chunk_id: u32,
        byte_offset: u32,
    }

    #[derive(serde::Serialize, Debug)]
    struct ExtraItem {
        tag: String,
        namespace: String,
        body_len: usize,
    }

    let head = p.head();
    let toc: Vec<_> = p
        .toc_entries()
        .map(|e| TocItem {
            chunk_type: e.chunk_type_str().to_string(),
            chunk_id: e.chunk_id,
            offset: e.offset,
            size_compressed: e.size_compressed,
            size_raw: e.size_raw,
            compression: e.compression as u8,
            content_type_kind: e.content_type_kind,
            content_type_value: e.content_type_value,
            cover_type: e.cover_type as u8,
            flags: e.flags,
            crc32: e.crc32,
            alt_text: e.alt_text.map(|s| s.to_string()),
            is_encrypted: e.is_encrypted(),
        })
        .collect();

    let pmap: Vec<_> = p
        .pmap_entries()
        .map(|e| PmapItem {
            print_page: e.print_page,
            chunk_id: e.chunk_id,
            byte_offset: e.byte_offset,
        })
        .collect();

    let meta_value = p
        .meta_bytes()
        .ok()
        .and_then(|b| rmp_serde::from_slice::<HonzoMeta>(b).ok())
        .map(|m| serde_json::to_value(m).unwrap_or(serde_json::Value::Null))
        .unwrap_or(serde_json::Value::Null);

    let extra_entries = p
        .extra_bytes()
        .ok()
        .map(|b| {
            honzo_io::parse_extra(b)
                .unwrap_or_default()
                .into_iter()
                .map(|e| ExtraItem {
                    tag: core::str::from_utf8(&e.tag).unwrap_or("?").to_string(),
                    namespace: e.namespace,
                    body_len: e.body.len(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let dump = Dump {
        path: file.to_string_lossy().to_string(),
        version_major: head.version_major,
        version_minor: head.version_minor,
        min_reader_version: head.min_reader_version,
        flags: head.flags,
        chunk_count: head.chunk_count,
        toc_size: head.toc_size,
        data_size: head.data_size,
        extra_size: head.extra_size,
        meta_size: head.meta_size,
        layout_mode: head.layout_mode() as u8,
        has_drm: head.has_drm(),
        has_sidx: head.has_sidx(),
        has_anno: head.has_anno(),
        has_sync: head.has_sync(),
        toc,
        pmap,
        meta: meta_value,
        extra_entries,
    };

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&dump).expect("failed to serialize dump")
        );
    } else {
        println!("{:#?}", dump);
    }
}

fn cmd_extract(file: &Path, chunk_id: u32, out: &Path) {
    let data = read_file_or_exit(file);
    let p = honzo_core::HonzoParser::new(&data, 1).unwrap_or_else(|e| {
        eprintln!("Parse error: {:?}", e);
        std::process::exit(1);
    });

    let entry = p.find_chunk_by_id(chunk_id).unwrap_or_else(|| {
        eprintln!("Error: chunk {} not found", chunk_id);
        std::process::exit(1);
    });

    if entry.is_encrypted() {
        eprintln!("Error: chunk {} is encrypted", chunk_id);
        std::process::exit(1);
    }

    let raw = p.chunk_bytes(&entry).unwrap_or_else(|e| {
        eprintln!("Error reading chunk: {:?}", e);
        std::process::exit(1);
    });
    let decompressed = decompress(raw, entry.compression, entry.size_raw).unwrap_or_else(|e| {
        eprintln!("Decompression error: {:?}", e);
        std::process::exit(1);
    });
    fs::write(out, &decompressed).unwrap_or_else(|e| {
        eprintln!("Error writing {}: {}", out.display(), e);
        std::process::exit(1);
    });
    println!(
        "Extracted chunk {} -> {} ({} bytes)",
        chunk_id,
        out.display(),
        decompressed.len()
    );
}

fn cmd_extract_all(file: &Path, out_dir: &Path) {
    let data = read_file_or_exit(file);
    let p = honzo_core::HonzoParser::new(&data, 1).unwrap_or_else(|e| {
        eprintln!("Parse error: {:?}", e);
        std::process::exit(1);
    });

    fs::create_dir_all(out_dir).unwrap_or_else(|e| {
        eprintln!("Error creating directory {}: {}", out_dir.display(), e);
        std::process::exit(1);
    });

    for entry in p.toc_entries() {
        let tag = entry.chunk_type_str();
        let raw = p.chunk_bytes(&entry).unwrap_or_else(|e| {
            eprintln!("Error reading chunk {}: {:?}", entry.chunk_id, e);
            std::process::exit(1);
        });
        let data = decompress(raw, entry.compression, entry.size_raw).unwrap_or_else(|e| {
            eprintln!("Decompression error chunk {}: {:?}", entry.chunk_id, e);
            std::process::exit(1);
        });
        let ext = match tag {
            "CHAP" | "NOTE" => "xhtml",
            "IMG_" | "COVR" | "COVT" => "bin",
            "CSS_" => "css",
            "FONT" => "bin",
            "SIDX" => "msgpack",
            "MATH" => "xml",
            _ => "bin",
        };
        let filename = format!("{}_{}_{}", entry.chunk_id, tag, ext);
        fs::write(out_dir.join(&filename), &data).unwrap_or_else(|e| {
            eprintln!("Error writing {}: {}", filename, e);
            std::process::exit(1);
        });
        println!("  {}", filename);
    }
}

fn cmd_build(spec: &Path, out: &Path) {
    let spec_data = read_file_or_exit(spec);
    let spec: serde_json::Value = serde_json::from_slice(&spec_data).unwrap_or_else(|e| {
        eprintln!("Error parsing spec JSON: {}", e);
        std::process::exit(1);
    });

    let mut builder = HonzoBuilder::new();

    if let Some(chunks) = spec["chunks"].as_array() {
        for chunk in chunks {
            let tag_str = chunk["tag"].as_str().unwrap_or("CHAP");
            if tag_str.len() != 4 {
                eprintln!("Error: invalid tag '{}' (must be 4 chars)", tag_str);
                std::process::exit(1);
            }
            let mut tag = [0u8; 4];
            tag.copy_from_slice(tag_str.as_bytes());

            let data_str = chunk["data"].as_str().unwrap_or("");
            let data = data_str.as_bytes().to_vec();

            let compression = match chunk["compression"].as_u64().unwrap_or(0) {
                0 => Compression::None,
                1 => Compression::Lz4,
                _ => {
                    eprintln!("Error: invalid compression");
                    std::process::exit(1);
                }
            };
            let kind = chunk["content_type_kind"].as_u64().unwrap_or(0) as u8;
            let value = chunk["content_type_value"].as_u64().unwrap_or(0) as u8;
            if &tag == b"MATH" {
                if kind != 2 {
                    eprintln!("Error: invalid content_type_kind for MATH");
                    std::process::exit(1);
                }
                let math = match value {
                    0 => honzo_core::MathType::MathML,
                    1 => honzo_core::MathType::LaTeX,
                    _ => {
                        eprintln!("Error: invalid content_type_value for MATH");
                        std::process::exit(1);
                    }
                };
                builder = builder.add_math_chunk(&data, math, compression);
            } else {
                if kind != 1 {
                    eprintln!("Error: invalid content_type_kind for markup chunk");
                    std::process::exit(1);
                }
                let markup = match value {
                    0 => MarkupType::Markdown,
                    1 => MarkupType::Html,
                    _ => {
                        eprintln!("Error: invalid content_type_value");
                        std::process::exit(1);
                    }
                };
                builder = builder.add_chunk(
                    tag,
                    &data,
                    compression,
                    markup,
                    CoverType::Front,
                    None,
                    None,
                    None,
                );
            }
        }
    }

    if let Some(meta_val) = spec.get("meta") {
        let meta: HonzoMeta = serde_json::from_value(meta_val.clone()).unwrap_or_else(|e| {
            eprintln!("Error parsing meta: {}", e);
            std::process::exit(1);
        });
        let msgpack = rmp_serde::to_vec(&meta).unwrap_or_else(|e| {
            eprintln!("Error serializing meta: {}", e);
            std::process::exit(1);
        });
        builder = builder.set_meta(&msgpack);
    }

    let hzo = builder.finalize().unwrap_or_else(|e| {
        eprintln!("Build error: {:?}", e);
        std::process::exit(1);
    });
    fs::write(out, &hzo).unwrap_or_else(|e| {
        eprintln!("Error writing {}: {}", out.display(), e);
        std::process::exit(1);
    });
    println!("Built {} ({} bytes)", out.display(), hzo.len());
}

fn detect_format(data: &[u8]) -> &'static str {
    if data.len() > 4 && &data[..4] == b"PK\x03\x04" {
        "epub"
    } else if data.len() > 4 && &data[..4] == b"%PDF" {
        "pdf"
    } else if (data.len() > 68 && &data[0x3C..0x40] == b"MOBI")
        || (data.len() > 4 && &data[..4] == b"BOOK")
    {
        "mobi"
    } else {
        "mobi/azw3"
    }
}

fn cmd_convert(input: &Path, out: &Path) {
    // Directory mode: convert markdown project
    if input.is_dir() {
        eprintln!("Detected format: markdown (directory)");
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg}")
                .expect("invalid progress template")
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        spinner.set_message("Converting...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));
        let result = honzo_convert::from_markdown_dir(input);
        spinner.finish_and_clear();
        match result {
            Ok(hzo) => {
                fs::write(out, &hzo).unwrap_or_else(|e| {
                    eprintln!("Error writing {}: {}", out.display(), e);
                    std::process::exit(1);
                });
                println!(
                    "Converted {} -> {} ({} bytes)",
                    input.display(),
                    out.display(),
                    hzo.len()
                );
            }
            Err(e) => {
                eprintln!("Conversion failed: {:?}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Markdown file mode
    let ext = input.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown") {
        eprintln!("Detected format: markdown (file)");
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg}")
                .expect("invalid progress template")
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        spinner.set_message("Converting...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(80));
        let result = honzo_convert::from_markdown_file(input);
        spinner.finish_and_clear();
        match result {
            Ok(hzo) => {
                fs::write(out, &hzo).unwrap_or_else(|e| {
                    eprintln!("Error writing {}: {}", out.display(), e);
                    std::process::exit(1);
                });
                println!(
                    "Converted {} -> {} ({} bytes)",
                    input.display(),
                    out.display(),
                    hzo.len()
                );
            }
            Err(e) => {
                eprintln!("Conversion failed: {:?}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Existing magic-byte detection for epub/mobi/pdf
    let data = read_file_or_exit(input);
    let detected = detect_format(&data);
    eprintln!("Detected format: {}", detected);

    let start = Instant::now();
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .expect("invalid progress template")
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    let tracker = ProgressTracker::new(pb.clone());

    let result = match detected {
        "epub" => honzo_convert::from_epub_with_progress(&data, &tracker),
        "pdf" => honzo_convert::from_pdf(&data),
        _ => honzo_convert::from_mobi(&data),
    };
    pb.finish_and_clear();

    match result {
        Ok(hzo) => {
            fs::write(out, &hzo).unwrap_or_else(|e| {
                eprintln!("Error writing {}: {}", out.display(), e);
                std::process::exit(1);
            });
            println!(
                "Converted {} -> {} ({}, {})",
                input.display(),
                out.display(),
                human_size(hzo.len() as u64),
                HumanDuration(start.elapsed())
            );
        }
        Err(e) => {
            eprintln!("Conversion failed (detected format: {}): {:?}", detected, e);
            std::process::exit(1);
        }
    }
}

fn cmd_convert_batch(pattern: &str, out_dir: &Path) {
    fs::create_dir_all(out_dir).unwrap_or_else(|e| {
        eprintln!(
            "Error creating output directory {}: {}",
            out_dir.display(),
            e
        );
        std::process::exit(1);
    });

    let files: Vec<PathBuf> = glob::glob(pattern)
        .unwrap_or_else(|e| {
            eprintln!("Error parsing pattern '{}': {}", pattern, e);
            std::process::exit(1);
        })
        .filter_map(|entry| entry.ok())
        .filter(|path| path.is_file())
        .filter(|path| {
            !path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("._"))
        })
        .collect();

    if files.is_empty() {
        eprintln!("No files matched pattern '{}'", pattern);
        return;
    }

    let mut seen_stems: std::collections::HashSet<String> = std::collections::HashSet::new();
    for input in &files {
        let stem = input
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        if !seen_stems.insert(stem.clone()) {
            eprintln!(
                "Error: multiple inputs resolve to the same output '{}', aborting batch",
                stem
            );
            std::process::exit(1);
        }
    }

    let total = files.len();
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{bar:30}] {pos}/{len} files ({elapsed_precise})",
        )
        .expect("invalid progress template")
        .progress_chars("=> "),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let start = Instant::now();
    let errors = AtomicU32::new(0);

    files.par_iter().for_each(|input| {
        let ext = input
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let result = if ext.eq_ignore_ascii_case("md") || ext.eq_ignore_ascii_case("markdown") {
            honzo_convert::from_markdown_file(input)
        } else {
            let data = match fs::read(input) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("Error reading {}: {}", input.display(), e);
                    errors.fetch_add(1, Ordering::Relaxed);
                    pb.inc(1);
                    return;
                }
            };

            let detected = detect_format(&data);

            match detected {
                "epub" => honzo_convert::from_epub(&data),
                "pdf" => honzo_convert::from_pdf(&data),
                _ => honzo_convert::from_mobi(&data),
            }
        };

        match result {
            Ok(hzo) => {
                let mut out_name = input.file_stem().unwrap_or_default().to_os_string();
                out_name.push(".hzo");
                let out_path = out_dir.join(out_name);
                if let Err(e) = fs::write(&out_path, &hzo) {
                    eprintln!("Error writing {}: {}", out_path.display(), e);
                    errors.fetch_add(1, Ordering::Relaxed);
                }
            }
            Err(e) => {
                eprintln!("  Conversion failed {}: {:?}", input.display(), e);
                errors.fetch_add(1, Ordering::Relaxed);
            }
        }
        pb.inc(1);
    });

    pb.finish_and_clear();
    let err_count = errors.load(Ordering::Relaxed);
    let converted = total as u32 - err_count;
    if err_count > 0 {
        eprintln!(
            "Batch convert finished: {} converted, {} errors ({})",
            converted,
            err_count,
            HumanDuration(start.elapsed())
        );
        std::process::exit(1);
    } else {
        println!(
            "Batch convert finished: {} converted ({})",
            converted,
            HumanDuration(start.elapsed())
        );
    }
}

fn cmd_validate(file: &Path) {
    let data = read_file_or_exit(file);
    let p = match honzo_core::HonzoParser::new(&data, 1) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            std::process::exit(1);
        }
    };

    let mut errors = 0u32;
    for entry in p.toc_entries() {
        if entry.chunk_type != *b"CHAP" {
            continue;
        }
        match p.chunk_bytes(&entry) {
            Ok(raw) => match decompress(raw, entry.compression, entry.size_raw) {
                Ok(decompressed) => {
                    if verify_entry_crc32(&entry, &decompressed).is_err() {
                        eprintln!("  CRC mismatch on chunk {}", entry.chunk_id);
                        errors += 1;
                    }
                }
                Err(_) => {
                    eprintln!("  Decompress error on chunk {}", entry.chunk_id);
                    errors += 1;
                }
            },
            Err(e) => {
                eprintln!("  Error reading chunk {}: {:?}", entry.chunk_id, e);
                errors += 1;
            }
        }
    }

    let count = p.toc_entries().count();
    if errors > 0 {
        eprintln!(
            "{}: INVALID ({} chunks, {} errors)",
            file.display(),
            count,
            errors
        );
        std::process::exit(1);
    } else {
        println!("{}: VALID ({} chunks)", file.display(), count);
    }
}

fn extract_excerpt(text: &str, byte_offset: u32, context: usize) -> String {
    let offset = byte_offset as usize;
    if offset > text.len() {
        return String::new();
    }

    let start = offset.saturating_sub(context);
    let end = (offset + context).min(text.len());

    let start = match text[start..].char_indices().next() {
        Some((i, _)) => start + i,
        None => start,
    };
    let end = match text[..end].char_indices().next_back() {
        Some((i, _)) => {
            let next_char = text[end..].chars().next();
            i + next_char.map(|c| c.len_utf8()).unwrap_or(0)
        }
        None => end,
    };

    let before = &text[start..offset];
    let match_word = &text[offset..std::cmp::min(offset + 40, text.len())];
    let match_end = match_word
        .find(|c: char| c.is_ascii_punctuation() || c.is_whitespace())
        .map(|i| offset + i)
        .unwrap_or(std::cmp::min(offset + 40, text.len()));

    let after_start = std::cmp::min(offset + 40, text.len());
    let after = &text[offset..after_start];
    let remaining = &text[after_start..end];

    let prefix = if start > 0 { "…" } else { "" };
    let suffix_ellipsis = if end < text.len() { "…" } else { "" };

    format!(
        "{}{}{}[MATCH]{}[\\MATCH]{}{}",
        prefix,
        before,
        &text[offset..match_end],
        after,
        remaining,
        suffix_ellipsis
    )
}

fn cmd_search(file: &Path, query: &str) {
    let data = read_file_or_exit(file);
    let p = honzo_core::HonzoParser::new(&data, 1).unwrap_or_else(|e| {
        eprintln!("Parse error: {:?}", e);
        std::process::exit(1);
    });

    if !p.head().has_sidx() {
        eprintln!("Error: no search index in this file");
        std::process::exit(1);
    }

    let lang = p
        .meta_bytes()
        .ok()
        .and_then(|b| rmp_serde::from_slice::<HonzoMeta>(b).ok())
        .map(|m| m.language)
        .unwrap_or_else(|| "en".to_string());

    let sidx_entry = p.find_chunk(b"SIDX").expect("SIDX chunk not found");
    let raw = p.chunk_bytes(&sidx_entry).unwrap_or_else(|e| {
        eprintln!("Error reading SIDX: {:?}", e);
        std::process::exit(1);
    });
    let decompressed =
        decompress(raw, sidx_entry.compression, sidx_entry.size_raw).unwrap_or_else(|e| {
            eprintln!("Error decompressing SIDX: {:?}", e);
            std::process::exit(1);
        });

    let index: HashMap<String, Vec<(u32, u32)>> = rmp_serde::from_slice(&decompressed)
        .unwrap_or_else(|e| {
            eprintln!("Error parsing SIDX: {:?}", e);
            std::process::exit(1);
        });

    // Support multi-term queries by normalizing each token and intersecting hits.
    let lang_ref = &lang;
    let terms: Vec<String> = query
        .split_whitespace()
        .map(|t| normalize_search_term(t, lang_ref))
        .filter(|s| !s.is_empty())
        .collect();

    if terms.is_empty() {
        println!("No results for '{}'", query);
        return;
    }

    let mut hits_by_chunk: HashMap<u32, (u32, Vec<u32>)> = HashMap::new();

    for term in &terms {
        if let Some(bucket) = index.get(term) {
            let mut seen_offsets: HashSet<(u32, u32)> = HashSet::new();
            let mut seen_chunks: HashSet<u32> = HashSet::new();
            for (chunk_id, offset) in bucket {
                let off_key = (*chunk_id, *offset);
                if !seen_offsets.contains(&off_key) {
                    seen_offsets.insert(off_key);
                    let entry = hits_by_chunk.entry(*chunk_id).or_insert((0u32, Vec::new()));
                    entry.1.push(*offset);
                }
                if !seen_chunks.contains(chunk_id) {
                    seen_chunks.insert(*chunk_id);
                    let entry = hits_by_chunk.entry(*chunk_id).or_insert((0u32, Vec::new()));
                    entry.0 += 1;
                }
            }
        } else {
            eprintln!("Term '{}' not found in index.", term);
            println!("No results for '{}'", query);
            return;
        }
    }

    let mut matches: Vec<(u32, u32, Vec<u32>)> = hits_by_chunk
        .into_iter()
        .filter(|(_, (score, _))| *score == terms.len() as u32)
        .map(|(chunk_id, (score, offsets))| (chunk_id, score, offsets))
        .collect();

    if matches.is_empty() {
        println!("No results for '{}'", query);
        return;
    }

    matches.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    // Build chunk id -> tag lookup
    let mut chunk_tags: HashMap<u32, String> = HashMap::new();
    for (chunk_id, _, _) in &matches {
        if !chunk_tags.contains_key(chunk_id) {
            if let Some(entry) = p.find_chunk_by_id(*chunk_id) {
                chunk_tags.insert(*chunk_id, entry.chunk_type_str().to_string());
            }
        }
    }

    // Pre-decompress all CHAP/NOTE chunks for excerpt extraction
    let mut chunk_texts: HashMap<u32, String> = HashMap::new();
    for (chunk_id, _, _) in &matches {
        if !chunk_texts.contains_key(chunk_id) {
            if let Some(entry) = p.find_chunk_by_id(*chunk_id) {
                if !entry.is_encrypted() {
                    if let Ok(raw) = p.chunk_bytes(&entry) {
                        if let Ok(decompressed) = decompress(raw, entry.compression, entry.size_raw)
                        {
                            if let Ok(text) = String::from_utf8(decompressed) {
                                chunk_texts.insert(*chunk_id, text);
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Found '{}' in {} chunk(s):", query, matches.len());
    for (chunk_id, score, offsets) in &matches {
        let tag = chunk_tags
            .get(chunk_id)
            .map(|s| s.as_str())
            .unwrap_or("????");
        for offset in offsets {
            let excerpt = chunk_texts
                .get(chunk_id)
                .map(|text| extract_excerpt(text, *offset, 50))
                .unwrap_or_default();
            println!(
                "  [{}] chunk {} at byte {} (score={})",
                tag, chunk_id, offset, score
            );
            if !excerpt.is_empty() {
                println!("    {}", excerpt);
            }
        }
    }
}

fn human_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

fn cmd_tree(file: &Path) {
    let data = read_file_or_exit(file);
    let p = honzo_core::HonzoParser::new(&data, 1).unwrap_or_else(|e| {
        eprintln!("Parse error: {:?}", e);
        std::process::exit(1);
    });
    let head = p.head();
    let total_size = data.len() as u64;

    let entries: Vec<_> = p.toc_entries().collect();
    let label = file
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_default()
        .to_string();
    println!(
        "{}  [{}.{} | {} chunks | {}]",
        label,
        head.version_major,
        head.version_minor,
        head.chunk_count,
        human_size(total_size),
    );

    let has_extra = head.extra_size > 0;

    // Top-level connectors
    // Order: HEAD, META, TOC, DATA[, EXTRA]
    let tl = |idx: usize, count: usize| {
        if idx == count - 1 {
            ("└", "    ")
        } else {
            ("├", "│   ")
        }
    };

    // HEAD
    let top_count = if has_extra { 5 } else { 4 };
    let (conn, child) = tl(0, top_count);
    println!("{conn} HEAD (48 B)");
    let flag_strs: Vec<&str> = [
        head.has_sidx().then_some("has_sidx"),
        head.has_drm().then_some("has_drm"),
        head.has_anno().then_some("has_anno"),
        head.has_sync().then_some("has_sync"),
    ]
    .into_iter()
    .flatten()
    .collect();
    let flag_line = if flag_strs.is_empty() {
        "none".to_string()
    } else {
        flag_strs.join(", ")
    };

    let head_fields = [
        format!("version: {}.{}", head.version_major, head.version_minor),
        format!("min_reader_version: {}", head.min_reader_version),
        format!("flags: {}", flag_line),
        format!("layout: {:?}", head.layout_mode()),
        format!("chunk_count: {}", head.chunk_count),
        format!("TOC: {} ({} B)", human_size(head.toc_size), head.toc_size),
        format!(
            "DATA: {} ({} B)",
            human_size(head.data_size),
            head.data_size
        ),
        format!(
            "EXTRA: {} ({} B)",
            human_size(head.extra_size),
            head.extra_size
        ),
        format!(
            "META: {} ({} B)",
            human_size(head.meta_size),
            head.meta_size
        ),
    ];
    let hf_count = head_fields.len();
    for (i, line) in head_fields.iter().enumerate() {
        let (hconn, _) = tl(i, hf_count);
        println!("{child}{hconn} {line}");
    }

    // META
    let (conn, child) = tl(1, top_count);
    println!("{conn} META ({})", human_size(head.meta_size));
    if let Ok(meta_bytes) = p.meta_bytes() {
        if let Ok(meta) = rmp_serde::from_slice::<HonzoMeta>(meta_bytes) {
            let title_str = meta
                .title
                .as_ref()
                .and_then(|t| {
                    t.get(&meta.language)
                        .or_else(|| t.values().next())
                        .map(|s| s.as_str())
                })
                .unwrap_or("?");
            let meta_lines: Vec<String> = {
                let mut ml = Vec::new();
                ml.push(format!("title: {} ({})", title_str, meta.language));
                if let Some(ref t) = meta.title {
                    if t.len() > 1 {
                        let langs: Vec<&str> = t.keys().map(|k| k.as_str()).collect();
                        ml.push(format!("translations: {}", langs.join(", ")));
                    }
                }
                for a in &meta.authors {
                    ml.push(format!("author: {}", a));
                }
                if let Some(ref wc) = meta.word_count {
                    ml.push(format!("words: {}", wc));
                }
                if let Some(ref rt) = meta.reading_time_mins {
                    ml.push(format!("reading_time: {} min", rt));
                }
                ml
            };
            let mc = meta_lines.len();
            for (i, line) in meta_lines.iter().enumerate() {
                let (mconn, _) = tl(i, mc);
                println!("{child}{mconn} {line}");
            }
        }
    }

    // TOC
    let (conn, _child) = tl(2, top_count);
    println!("{conn} TOC (entries: {})", entries.len());

    // DATA
    let (conn, child) = tl(3, top_count);
    println!("{conn} DATA ({})", human_size(head.data_size));

    // Group entries by tag type
    let mut groups: BTreeMap<&str, Vec<&TocEntry>> = BTreeMap::new();
    for entry in &entries {
        let tag = core::str::from_utf8(&entry.chunk_type).unwrap_or("????");
        groups.entry(tag).or_default().push(entry);
    }

    let group_count = groups.len();
    for (gi, (tag, group_entries)) in groups.iter().enumerate() {
        let is_last_group = gi == group_count - 1;
        let (gconn, gchild) = if is_last_group {
            ("└", "    ")
        } else {
            ("├", "│   ")
        };
        println!(
            "{child}{gconn} {tag} ({} chunk{})",
            group_entries.len(),
            if group_entries.len() == 1 { "" } else { "s" },
        );

        let ec = group_entries.len();
        for (ei, entry) in group_entries.iter().enumerate() {
            let is_last = ei == ec - 1;
            let (econn, _) = if is_last { ("└", "") } else { ("├", "") };

            let desc = match *tag {
                "CHAP" => entry.alt_text.unwrap_or("chapter"),
                "COVR" | "COVT" | "IMG_" => entry.alt_text.unwrap_or("image"),
                "CSS_" => "stylesheet",
                "FONT" => "font",
                "SIDX" => "search index",
                "MATH" => "math",
                "NOTE" => entry.alt_text.unwrap_or("note"),
                _ => "",
            };
            let ct = match *tag {
                "CHAP" | "NOTE" => match (entry.content_type_kind, entry.content_type_value) {
                    (1, 1) => Some("html"),
                    (1, 0) => Some("markdown"),
                    _ => None,
                },
                "MATH" => match (entry.content_type_kind, entry.content_type_value) {
                    (2, 0) => Some("mathml"),
                    (2, 1) => Some("latex"),
                    _ => None,
                },
                _ => None,
            };
            let compressed = if entry.compression != honzo_core::Compression::None {
                format!(
                    " [lz4: {}→{}]",
                    human_size(entry.size_compressed as u64),
                    human_size(entry.size_raw as u64)
                )
            } else {
                String::new()
            };
            let qualifier = match ct {
                Some(c) => format!(" ({})", c),
                None => String::new(),
            };

            println!(
                "{child}{gchild}{econn} [{:>3}] {}{}{}",
                entry.chunk_id, desc, qualifier, compressed,
            );
        }
    }

    // EXTRA
    if has_extra {
        let (conn, _child) = tl(4, top_count);
        println!("{conn} EXTRA ({})", human_size(head.extra_size));
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Info { file } => cmd_info(&file),
        Commands::Inspect { file, json } => cmd_inspect(&file, json),
        Commands::Extract { file, chunk, out } => cmd_extract(&file, chunk, &out),
        Commands::ExtractAll { file, out_dir } => cmd_extract_all(&file, &out_dir),
        Commands::Build { spec, out } => cmd_build(&spec, &out),
        Commands::Convert { input, out } => cmd_convert(&input, &out),
        Commands::ConvertBatch { pattern, out_dir } => cmd_convert_batch(&pattern, &out_dir),
        Commands::Validate { file } => cmd_validate(&file),
        Commands::Search { file, query } => cmd_search(&file, &query),
        Commands::Tree { file } => cmd_tree(&file),
    }
}
