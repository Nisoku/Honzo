use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use honzo_std::*;

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
    Convert {
        input: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Parse and validate .hzo file
    Validate { file: PathBuf },
    /// Query SIDX search index
    Search {
        file: PathBuf,
        #[arg(long)]
        query: String,
    },
}

fn read_file(path: &PathBuf) -> Vec<u8> {
    let mut f = fs::File::open(path).unwrap_or_else(|e| {
        eprintln!("Error: cannot open {}: {}", path.display(), e);
        std::process::exit(1);
    });
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap_or_else(|e| {
        eprintln!("Error: cannot read {}: {}", path.display(), e);
        std::process::exit(1);
    });
    buf
}

fn cmd_info(file: &PathBuf) {
    let data = read_file(file);
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

fn cmd_inspect(file: &PathBuf, json: bool) {
    let data = read_file(file);
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
        markup_type: u8,
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
            markup_type: e.markup_type as u8,
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
            honzo_std::parse_extra(b)
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
        println!("{}", serde_json::to_string_pretty(&dump).unwrap());
    } else {
        println!("{:#?}", dump);
    }
}

fn cmd_extract(file: &PathBuf, chunk_id: u32, out: &PathBuf) {
    let data = read_file(file);
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

fn cmd_extract_all(file: &PathBuf, out_dir: &PathBuf) {
    let data = read_file(file);
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

fn cmd_build(spec: &PathBuf, out: &PathBuf) {
    let spec_data = read_file(spec);
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
                1 => Compression::Zlib,
                2 => Compression::Zstd,
                _ => {
                    eprintln!("Error: invalid compression");
                    std::process::exit(1);
                }
            };
            let markup = match chunk["markup_type"].as_u64().unwrap_or(0) {
                0 => MarkupType::Hmd,
                1 => MarkupType::Html,
                _ => {
                    eprintln!("Error: invalid markup_type");
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

fn cmd_convert(input: &PathBuf, out: &PathBuf) {
    let data = read_file(input);

    let detected = if data.len() > 4 && &data[..4] == b"PK\x03\x04" {
        "epub"
    } else if data.len() > 4 && &data[..4] == b"%PDF" {
        "pdf"
    } else {
        "mobi/azw3"
    };
    eprintln!("Detected format: {}", detected);

    let result = if detected == "epub" {
        honzo_convert::from_epub(&data)
    } else if detected == "pdf" {
        honzo_convert::from_pdf(&data)
    } else {
        honzo_convert::from_mobi(&data)
    };

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
}

fn cmd_validate(file: &PathBuf) {
    let data = read_file(file);
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

fn cmd_search(file: &PathBuf, query: &str) {
    let data = read_file(file);
    let p = honzo_core::HonzoParser::new(&data, 1).unwrap_or_else(|e| {
        eprintln!("Parse error: {:?}", e);
        std::process::exit(1);
    });

    if !p.head().has_sidx() {
        eprintln!("Error: no search index in this file");
        std::process::exit(1);
    }

    let sidx_entry = p.find_chunk(b"SIDX").unwrap();
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

    let query_lower = query.to_ascii_lowercase();
    if let Some(results) = index.get(&query_lower) {
        println!("Found '{}' in {} location(s):", query, results.len());
        for (chunk_id, byte_offset) in results {
            println!("  chunk {} at byte offset {}", chunk_id, byte_offset);
        }
    } else {
        println!("No results for '{}'", query);
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
        Commands::Validate { file } => cmd_validate(&file),
        Commands::Search { file, query } => cmd_search(&file, &query),
    }
}
