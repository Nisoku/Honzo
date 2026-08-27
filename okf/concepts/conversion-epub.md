---
type: concept
title: "EPUB Conversion"
description: "Converting EPUB 2/3 files to Honzo format"
source: "https://nisoku.org/Honzo/conversion/epub/"
path: /conversion/epub/
updated: 2026-08-27
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-27T06:57:57.071Z"
---
---
title: "EPUB Conversion"
description: "Converting EPUB 2/3 files to Honzo format"
---

EPUB conversion is the most complete conversion path in Honzo. It handles EPUB 2 and 3, including the navigation document, metadata, embedded resources, and page breaks.

## How it works

::: steps

1. **Unpack.** EPUB is a ZIP container. The converter extracts the OPF manifest and NCX or nav document.

2. **Parse metadata.** Title, creator, language, identifiers, subjects, and series info come from the OPF.

3. **Build spine.** The EPUB spine, which defines reading order, becomes the Honzo TOC and CHAP entries.

4. **Embed resources.** Images, CSS, and fonts referenced by the XHTML are embedded as IMG_, CSS_, and FONT chunks.

5. **Detect page breaks.** EPUB pagebreak markers and character count heuristics produce page map data.

6. **Build.** All chunks are assembled into the final Honzo file with per-chapter LZ4 compression.

:::

## Metadata mapping

| EPUB field                        | Honzo META field                           |
| --------------------------------- | ------------------------------------------ |
| `dc:title`                        | `title` (multi-language if multiple)       |
| `dc:creator`                      | `creator`                                  |
| `dc:language`                     | `language`                                 |
| `dc:identifier`                   | `identifiers.isbn` (or `identifiers.uuid`) |
| `dc:subject`                      | `subject[]`                                |
| `dc:publisher`                    | `publisher`                                |
| `dc:date`                         | `published`                                |
| `dc:rights`                       | `rights`                                   |
| `meta property="series"`          | `series`                                   |
| `meta property="series_position"` | `series_position`                          |
| `meta property="edition"`         | `edition`                                  |
| `page-progression-direction`      | `page_progression_direction`               |

## Preserved features

::: grids
::: grid
::: card "Inline images" icon:image
`IMG_` chunks from embedded XHTML images.
:::
:::
::: grid
::: card "Stylesheets" icon:file-text
`CSS_` chunks preserve EPUB styling.
:::
:::
::: grid
::: card "Embedded fonts" icon:type
`FONT` chunks carry over WOFF2/TTF/OTF.
:::
:::
::: grid
::: card "Cover" icon:image
`COVR` + `COVT` chunks for full cover and thumbnail.
:::
:::
::: grid
::: card "Page breaks" icon:book-open
Detected and preserved in `EXTRA` section.
:::
:::
::: grid
::: card "MathML" icon:sigma
Converted to `MATH` chunks.
:::
:::
::: grid
::: card "Multi-language metadata" icon:globe
Preserved per BCP 47 language tags.
:::
:::
::: grid
::: card "Table of contents" icon:list
Honzo TOC + META toc array.
:::
:::
:::

::: callout warning "Limitations"

- EPUB scripting in JavaScript is not carried over.
- EPUB media overlays using SMIL are not converted; use `org.nisoku.sync` instead.
- Fixed layout EPUBs are converted as reflowable, though the layout mode can be overridden.

:::

## Example

```bash
# Basic conversion
honzo-cli convert book.epub book.hzo

# Override title and author
honzo-cli convert book.epub book.hzo \
  --title "My Edition" \
  --author "My Name" \
  --language "en"

# Inspect the result
honzo-cli info book.hzo
honzo-cli info --json book.hzo
```

## Next Steps

::: grids
::: grid
::: button "MOBI Conversion" ./mobi.md icon:book-open
:::
::: grid
::: button "PDF Conversion" ./pdf.md icon:file-text
:::
:::
