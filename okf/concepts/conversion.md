---
type: concept
title: Conversion
description: "Convert existing ebook formats to Honzo"
source: "https://nisoku.org/Honzo/conversion/"
path: /conversion/
updated: 2026-08-27
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-27T06:28:21.188Z"
---
---
title: "Conversion"
description: "Convert existing ebook formats to Honzo"
---

Honzo includes conversion tools for EPUB, MOBI, and PDF formats. Conversion preserves chapter structure, metadata, images, stylesheets, and fonts where possible.

## Supported formats

::: grids
::: grid
::: card "EPUB 2/3" icon:book
Table of contents, metadata, images, CSS, and fonts supported.
:::
:::
::: grid
::: card "MOBI" icon:book-open
Text, metadata, and basic formatting.
:::
:::
::: grid
::: card "PDF" icon:file-text
Text extraction only. No reflow preservation.
:::
:::
:::

## Converting files

::: tabs

== tab "CLI"

```bash
# EPUB to Honzo
honzo-cli convert book.epub book.hzo

# MOBI to Honzo
honzo-cli convert book.mobi book.hzo

# PDF to Honzo
honzo-cli convert book.pdf book.hzo
```

== tab "Rust API"

```rust
use honzo_convert::convert_epub;

let hzo = convert_epub("book.epub").unwrap();
std::fs::write("book.hzo", hzo).unwrap();
```

:::

## What gets converted

### EPUB

| Source        | Honzo target              | Details                                         |
| ------------- | ------------------------- | ----------------------------------------------- |
| OPF metadata  | META section              | Title, creator, language, identifiers, subjects |
| NCX / nav     | CHAP chunks + TOC         | Chapter splitting follows the EPUB spine        |
| XHTML content | CHAP chunks (HTML)        | Full HTML preserved, including in-line images   |
| Images        | IMG_ chunks               | JPEG, PNG, WebP embedded                        |
| CSS           | CSS_ chunks               | Stylesheets preserved                           |
| Fonts         | FONT chunks               | Embedded fonts carried over                     |
| Cover         | COVR + COVT               | Full cover + thumbnail                          |
| Page breaks   | EXTRA (via pagebreaks.rs) | EPUB pagebreak markers converted                |

### MOBI

| Source       | Honzo target       | Details                 |
| ------------ | ------------------ | ----------------------- |
| Metadata     | META section       | Title, author, language |
| Text content | CHAP chunks (HTML) | Basic HTML conversion   |
| Images       | IMG_ chunks        | Embedded images         |

### PDF

| Source         | Honzo target           | Details                           |
| -------------- | ---------------------- | --------------------------------- |
| Extracted text | CHAP chunks (Markdown) | Text flow, no layout preservation |
| Page breaks    | Chapter splitting      | One PDF page per chapter          |

::: collapsible "Page break detection"

For EPUBs without explicit pagebreak markers, Honzo uses a character count heuristic:

```rust
use honzo_convert::pagebreaks::estimate_page_breaks;

let breaks = estimate_page_breaks(&content, None);
// Returns around 2000 character intervals for English text
```

Explicit EPUB pagebreak patterns are also detected:

```html
<!-- These are all recognized -->
<span epub:type="pagebreak" id="pg42" title="42" />
<span class="pagebreak" id="page-42" />
<a id="page42" class="pagebreak"></a>
<hr class="pagebreak" />
<div class="pagebreak" title="42" />
```

:::

## Conversion options

Override metadata during conversion:

```bash
honzo-cli convert book.epub book.hzo \
  --title "Custom Title" \
  --author "Custom Author" \
  --language "fr"
```

## Next Steps

::: grids
::: grid
::: button "Quick Start" ../getting-started/quickstart.md icon:play
:::
::: grid
::: button "Format Specification" ../format/ icon:book
:::
:::

## Detailed guides

For format-specific conversion details:

::: grids
::: grid
::: card "EPUB" icon:book
Full-featured EPUB 2/3 converter with metadata, images, CSS, and fonts.

::: button "Read more" ./epub.md icon:arrow-right
:::
:::
::: grid
::: card "MOBI" icon:book-open
Amazon Kindle format import with text and basic formatting.

::: button "Read more" ./mobi.md icon:arrow-right
:::
:::
::: grid
::: card "PDF" icon:file-text
Basic text extraction. No reflow or layout preservation.

::: button "Read more" ./pdf.md icon:arrow-right
:::
:::
:::
