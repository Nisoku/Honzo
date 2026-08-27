---
type: concept
title: "Quick Start"
description: "Create, read, and inspect a Honzo file in 5 minutes"
source: "https://nisoku.org/Honzo/getting-started/quickstart/"
path: /getting-started/quickstart/
updated: 2026-08-27
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-27T06:57:57.080Z"
---
---
title: "Quick Start"
description: "Create, read, and inspect a Honzo file in 5 minutes"
---

This guide gets you from zero to a working Honzo file. You will build a simple ebook, inspect its structure, and read it back.

::: callout tip "Prerequisites"
You need Rust 1.75+ or Node.js 18+. See [Installation](./installation) to set things up.
:::

::: steps

1. **Create a Honzo File**

   ::: tabs

   == tab "Rust"

   ```rust
   use honzo_io::{HonzoBuilder, Compression, MarkupType};

   let hzo = HonzoBuilder::new()
       .meta_title("en", "My First Book")
       .meta_creator("en", "Author Name")
       .add_chapter("Chapter 1: The Beginning", Compression::Lz4, MarkupType::Markdown)
       .add_chapter("Chapter 2: The Middle", Compression::Lz4, MarkupType::Markdown)
       .add_chapter("Chapter 3: The End", Compression::None, MarkupType::Markdown)
       .finalize()
       .unwrap();

   std::fs::write("my_book.hzo", hzo).unwrap();
   println!("Created my_book.hzo");
   ```

   == tab "CLI"

   ```bash
   honzo-cli make my_book.hzo \
     --title "My First Book" \
     --author "Author Name" \
     --chapters chapter1.md chapter2.md chapter3.md
   ```

   :::

2. **Inspect the File**

   ::: tabs

   == tab "Rust"

   ```rust
   use honzo_core::HonzoParser;

   let data = std::fs::read("my_book.hzo").unwrap();
   let p = HonzoParser::new(&data, 1).unwrap();

   let head = p.head();
   println!("Format version: {}", head.format_version);
   println!("Chunk count:    {}", head.chunk_count);
   println!("Layout mode:    {}", head.layout_mode);

   for entry in p.toc_entries() {
       let tag = std::str::from_utf8(&entry.chunk_type).unwrap();
       let compressed = if entry.compression == 1 { " (lz4)" } else { "" };
       println!("  {tag}: size={}{}", entry.size, compressed);
   }
   ```

   == tab "CLI"

   ```bash
   honzo-cli info my_book.hzo
   ```

   Example output:

   ```txt
   Honzo File: my_book.hzo
     Format version: 1
     Chunk count:    5
     Layout mode:    reflowable
     Chunks:
       CHAP  size=184  (lz4)
       COVR  size=24576
       CHAP  size=196  (lz4)
       CHAP  size=172  (none)
       META  size=128
   ```

   :::

3. **Read Chapters**

   ::: tabs

   == tab "Rust Parser"

   ```rust
   use honzo_core::HonzoParser;

   let data = std::fs::read("my_book.hzo").unwrap();
   let p = HonzoParser::new(&data, 1).unwrap();

   for entry in p.toc_entries() {
       if &entry.chunk_type == b"CHAP" {
           let chapter_data = p.read_chunk(&entry).unwrap();
           let text = std::str::from_utf8(&chapter_data).unwrap();
           println!("Chapter: {text}");
       }
   }
   ```

   == tab "Rust Streaming"

   ```rust
   use honzo_io::HonzoStream;

   let file = std::fs::File::open("my_book.hzo").unwrap();
   let mut stream = HonzoStream::open(file, 1).unwrap();

   for chapter in stream.chapters() {
       let (text, _meta) = chapter.unwrap();
       println!("Chapter: {} bytes", text.len());
   }
   ```

   == tab "TypeScript"

   ```typescript
   import { createReader } from "@nisoku/honzo";

   const response = await fetch("my_book.hzo");
   const buf = new Uint8Array(await response.arrayBuffer());
   const reader = await createReader(buf);

   const meta = reader.getMeta();
   console.log("Title:", meta.title?.en);
   console.log("Chapters:", reader.chunkCount);

   for (let i = 0; i < reader.chunkCount; i++) {
     const chunk = reader.readChunk(i);
     const text = new TextDecoder().decode(chunk);
     console.log("Chapter:", text.substring(0, 100));
   }
   ```

   :::

4. **Convert from EPUB**

   ```bash
   honzo-cli convert book.epub book.hzo
   ```

   Or with Rust:

   ```rust
   use honzo_convert::convert_epub;

   let hzo = convert_epub("book.epub").unwrap();
   std::fs::write("book.hzo", hzo).unwrap();
   ```

:::

## Next Steps

::: grids
::: grid
::: button "Installation" ./installation.md icon:download
:::
::: grid
::: button "Core Concepts" ./concepts.md icon:book
:::
::: grid
::: button "Wire Format" ../format/wire-format.md icon:binary
:::
::: grid
::: button "CLI Reference" ../cli/ icon:terminal
:::
:::
