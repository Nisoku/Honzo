---
type: concept
title: "Chunk Types"
description: "All Honzo chunk type tags and their semantics"
source: "https://nisoku.org/Honzo/format/chunk-types/"
path: /format/chunk-types/
updated: 2026-08-30
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-30T03:35:43.356Z"
---
---
title: "Chunk Types"
description: "All Honzo chunk type tags and their semantics"
---

Each TOC entry carries a 4-byte ASCII type tag. This page documents every tag and describes the data each chunk contains.

## CHAP Chapter

**Tag:** ::: tag "CHAP" `0x43484150`

The book's chapter content. The data is either Markdown or HTML text. The `markup_type` field in the TOC entry determines which format.

| TOC field     | Description                                                        |
| ------------- | ------------------------------------------------------------------ |
| `markup_type` | ::: tag "0" color:#22c55e Markdown, ::: tag "1" color:#3b82f6 HTML |
| `compression` | Typically ::: tag "1" color:#22c55e (LZ4) for text chapters        |
| `language`    | BCP 47 index for chapter language                                  |

```text
Chapter data: UTF-8 encoded Markdown or HTML string
```

## IMG_ Image

**Tag:** ::: tag "IMG_" `0x494D475F`

::: callout info
The trailing underscore distinguishes this tag from IMG-related variant tags.
:::

An inline image referenced from chapter content.

| TOC field      | Description                                                 |
| -------------- | ----------------------------------------------------------- |
| `compression`  | Typically ::: tag "0" (none) for already compressed formats |
| `content_type` | Image format hint for JPEG, PNG, or WebP                    |

```text
Image data: raw image bytes in JPEG, PNG, or WebP format
```

The content type is validated by magic bytes on read. Supported formats include JPEG, PNG, and WebP.

## CSS_ Stylesheet

**Tag:** ::: tag "CSS_" `0x4353535F`

A CSS stylesheet for rendering chapter content.

| TOC field     | Description                          |
| ------------- | ------------------------------------ |
| `compression` | Typically ::: tag "1" (LZ4) for text |

```text
CSS data: UTF-8 encoded CSS string
```

The CSS is parsed and validated with `cssparser` on write.

## FONT Font

**Tag:** ::: tag "FONT" `0x464F4E54`

An embedded font file.

| TOC field              | Description                           |
| ---------------------- | ------------------------------------- |
| `font_embedding`       | Embedding mode such as subset or full |
| `font_license_url_len` | Length of embedded license URL        |
| `compression`          | Typically ::: tag "0" (none)          |

```text
Font data: raw font bytes in WOFF2, TTF, or OTF format
```

The font format is detected by magic bytes:

| Magic bytes                  | Format                |
| ---------------------------- | --------------------- |
| `wOF2` or `\x00\x01\x00\x00` | WOFF2                 |
| `OTTO`                       | OTF with CFF outlines |
| `\x00\x01\x00\x00`           | TTF                   |
| `true`                       | TTF in Mac format     |

The font license URL is a UTF-8 string stored as a variable-length field following the standard TOC fields.

## COVR Cover Image

**Tag:** ::: tag "COVR" `0x434F5652`

The full-size book cover image.

| TOC field     | Description                                                                 |
| ------------- | --------------------------------------------------------------------------- |
| `cover_type`  | ::: tag "1" color:#22c55e front cover, ::: tag "2" color:#3b82f6 back cover |
| `compression` | Typically ::: tag "0" (none)                                                |

```text
Cover data: raw image bytes in JPEG, PNG, or WebP format
```

## COVT Cover Thumbnail

**Tag:** ::: tag "COVT" `0x434F5654`

An optional downsized version of COVR for quick previews in listing views or ebook reader shelves.

| TOC field     | Description                                                                 |
| ------------- | --------------------------------------------------------------------------- |
| `cover_type`  | ::: tag "1" color:#22c55e front cover, ::: tag "2" color:#3b82f6 back cover |
| `compression` | Typically ::: tag "0" (none)                                                |

```text
Thumbnail data: raw image bytes in JPEG, PNG, or WebP format
```

## NOTE Note or Annotation

**Tag:** ::: tag "NOTE" `0x4E4F5445`

Embedded annotations associated with the file.

```text
Note data: application-defined format
```

This chunk type stores per-file annotation data. For a richer annotation model that travels with the file, see the `org.nisoku.anno` EXTRA namespace.

## SIDX Search Index

**Tag:** ::: tag "SIDX" `0x53494458`

An inverted search index for the book's content.

::: collapsible "SIDX MessagePack structure"

```text
{
  "version": 1,
  "terms": {
    "word": [
      {"chunk": 0, "positions": [42, 105]},
      {"chunk": 2, "positions": [17]}
    ],
    "another": [
      {"chunk": 1, "positions": [88]}
    ]
  }
}
```

:::

Terms are lowercase whitespace-split tokens. `chunk` refers to the TOC index of the chapter. `positions` are byte offsets within the decompressed chapter.

See the [Search Index feature](../features/search) for usage details.

## MATH Math Equation

**Tag:** ::: tag "MATH" `0x4D415448`

A math equation embedded in the book.

| TOC field      | Description                                           |
| -------------- | ----------------------------------------------------- |
| `content_type` | Math format: ::: tag "0" LaTeX, ::: tag "1" AsciiMath |

```text
Math data: UTF-8 encoded equation string
```

Equations are rendered at read time by the consuming application, for example with KaTeX or MathJax.

## Next Steps

::: grids
::: grid
::: button "Wire Format" ./wire-format.md icon:book
:::
::: grid
::: button "Compression" ./compression.md icon:archive
:::
::: grid
::: button "Layout Modes" ./layout.md icon:layout
:::
:::
