---
type: concept
title: "MOBI Conversion"
description: "Converting MOBI (Amazon Kindle) files to Honzo format"
source: "https://nisoku.org/Honzo/conversion/mobi/"
path: /conversion/mobi/
updated: 2026-08-27
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-27T06:42:02.791Z"
---
---
title: "MOBI Conversion"
description: "Converting MOBI (Amazon Kindle) files to Honzo format"
---

MOBI conversion provides partial import of Amazon Kindle format books. Text, metadata, and basic formatting carry over, but MOBI's proprietary features like Amazon's DRM and Kindle-specific markup are stripped.

## How it works

::: steps

1. **Parse MOBI header.** The Palm Database (PDB) header and MOBI header identify the encoding, compression, and metadata.

2. **Extract text.** The MOBI text is stored in PalmDoc format with LZ77 compression. The converter decompresses and re-encodes it as UTF-8 HTML.

3. **Parse metadata.** Title, author, and language come from the MOBI header and EXTH records.

4. **Split chapters.** MOBI chapter positions (from `srcs` / `srcc` records) divide the text into Honzo CHAP entries.

5. **Embed images.** MOBI images are extracted from the PDB records and embedded as `IMG_` chunks.

6. **Build.** All chunks are assembled into the final Honzo file with per-chapter LZ4 compression.

:::

## Metadata mapping

::: grids
::: grid
::: card "title" icon:book
Maps to Honzo `title` in `META` section.
:::
:::
::: grid
::: card "author" icon:user
Maps to Honzo `creator`.
:::
:::
::: grid
::: card "language" icon:globe
Maps to Honzo `language` (BCP 47).
:::
:::
::: grid
::: card "isbn" icon:hash
Maps to `identifiers.isbn`.
:::
:::
::: grid
::: card "publisher" icon:building
Maps to Honzo `publisher`.
:::
:::
::: grid
::: card "publishingdate" icon:calendar
Maps to Honzo `published`.
:::
:::
:::

## Preserved features

::: grids
::: grid
::: card "Chapter structure" icon:book
`CHAP` chunks via chapter break detection.
:::
:::
::: grid
::: card "Inline images" icon:image
`IMG_` chunks from PDB image records.
:::
:::
::: grid
::: card "Metadata" icon:info
Title, author, language in `META` section.
:::
:::
::: grid
::: card "Text formatting" icon:bold
Bold and italic preserved as HTML.
:::
:::
:::

::: callout warning "Limitations"

- Amazon DRM is not supported. Only DRM-free MOBI files can be converted.
- Kindle Format 8 (KF8) markup is partially preserved; some CSS-based formatting may be lost.
- Margin notes and highlights specific to the Kindle ecosystem are not carried over.
- Embedded fonts from AZW3/KF8 are not extracted.
- Page break data is inferred from MOBI chapter positions.

:::

## Example

```bash
honzo-cli convert book.mobi book.hzo
```

## Next Steps

::: grids
::: grid
::: button "EPUB Conversion" ./epub.md icon:book
:::
::: grid
::: button "PDF Conversion" ./pdf.md icon:file-text
:::
:::
