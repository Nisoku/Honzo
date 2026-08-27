---
type: concept
title: "PDF Conversion"
description: "Converting PDF files to Honzo format"
source: "https://nisoku.org/Honzo/conversion/pdf/"
path: /conversion/pdf/
updated: 2026-08-27
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-27T07:04:26.180Z"
---
---
title: "PDF Conversion"
description: "Converting PDF files to Honzo format"
---

PDF conversion provides basic text extraction from PDF files. PDF is fundamentally a print-oriented format with fixed layout, so the conversion focuses on recovering readable text rather than preserving visual positioning.

::: callout info
PDF conversion is the roughest path in Honzo. Images, formatting, tables, and fonts are not carried over. For structured documents, use EPUB instead.
:::

## How it works

::: steps

1. **Extract text.** PDF text content is extracted page by page using text show operators.

2. **Detect reading order.** The converter applies heuristics to reconstruct paragraph and chapter boundaries from positioned text.

3. **Split by pages.** Each PDF page becomes a separate Honzo CHAP entry.

4. **Extract metadata.** Title, author, and subject come from the PDF Info dictionary or XMP metadata.

5. **Build.** All extracted content is assembled into the final Honzo file.

:::

## Metadata mapping

::: grids
::: grid
::: card "Title" icon:book
Maps to Honzo `title` in `META` section.
:::
:::
::: grid
::: card "Author" icon:user
Maps to Honzo `creator`.
:::
:::
::: grid
::: card "Subject" icon:tag
Maps to `subject[]` array.
:::
:::
::: grid
::: card "Language" icon:globe
Maps to Honzo `language` (BCP 47).
:::
:::
:::

## Preserved features

::: grids
::: grid
::: card "Page structure" icon:book
One `CHAP` chunk per PDF page.
:::
:::
::: grid
::: card "Text content" icon:type
Extracted as Markdown text.
:::
:::
::: grid
::: card "Metadata" icon:info
Title, author preserved in `META`.
:::
:::
:::

::: callout warning "Limitations"

- PDF layout and formatting are not preserved. Expect plain text without bold, italic, or font styling.
- Images embedded in PDFs are not extracted.
- Tables and multi-column layouts may lose their structural relationships.
- Embedded fonts are not carried over.
- PDF forms, annotations, and interactive elements are ignored.
- OCR-based PDFs (scanned documents) produce no usable text output.

:::

## Example

```bash
honzo-cli convert book.pdf book.hzo
```

## Next Steps

::: grids
::: grid
::: button "EPUB Conversion" ./epub.md icon:book
:::
::: grid
::: button "MOBI Conversion" ./mobi.md icon:book-open
:::
:::
