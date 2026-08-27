---
type: concept
title: "CLI Reference"
description: "Command-line tools for working with Honzo files"
source: "https://nisoku.org/Honzo/cli/"
path: /cli/
updated: 2026-08-27
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-27T07:04:26.177Z"
---
---
title: "CLI Reference"
description: "Command-line tools for working with Honzo files"
---

The `honzo` CLI provides commands for creating, inspecting, converting, and managing Honzo files.

## Global usage

```bash
honzo-cli <command> [options]
```

| Flag        | Description  |
|-------------|--------------|
| `--help`    | Show help    |
| `--version` | Show version |

## Commands

::: card "**honzo-cli make**" icon:wrench
Create a Honzo file from markdown source files.

```bash
honzo-cli make <output.hzo> \
  --title "Book Title" \
  --author "Author Name" \
  --language "en" \
  --chapters chapter1.md chapter2.md chapter3.md
```

| Option                  | Description                                             |
|-------------------------|---------------------------------------------------------|
| `--title <text>`        | Book title                                              |
| `--author <text>`       | Book author                                             |
| `--language <code>`     | Language (BCP 47, default: `en`)                        |
| `--chapters <files...>` | Markdown files (each becomes a chapter)                 |
| `--cover <image>`       | Cover image file                                        |
| `--layout <mode>`       | `reflowable`, `fixed`, `scroll` (default: `reflowable`) |
| `--compress <algo>`     | `none`, `lz4` (default: `lz4`)                          |
| `--output, -o <file>`   | Output file (default: first positional arg)             |

:::

::: card "**honzo-cli info**" icon:info
Display metadata and structure of a Honzo file.

```bash
honzo-cli info <file.hzo>
```

Example output:

```txt
Honzo File: book.hzo
  Format version: 1
  Chunk count:    6
  Layout mode:    reflowable

Metadata:
  Title:    The Example Book
  Creator:  Author Name
  Language: en

Chunks:
  [0] CHAP  size=4,198  (lz4) - markdown
  [1] IMG_  size=24,576
  [2] CHAP  size=3,892  (lz4) - markdown
  [3] CSS_  size=1,024
  [4] CHAP  size=5,103  (lz4) - markdown
  [5] META  size=256
```

| Option          | Description              |
|-----------------|--------------------------|
| `--verbose, -v` | Show detailed TOC fields |
| `--json`        | Output as JSON           |

:::

::: card "**honzo-cli inspect**" icon:search
Low-level dump of the file structure.

```bash
honzo-cli inspect <file.hzo>
```

Shows HEAD fields, TOC entries (hex + decoded), and section offsets.
:::

::: card "**honzo-cli convert**" icon:git-merge
Convert an existing ebook format to Honzo.

```bash
honzo-cli convert <input> <output.hzo>
```

Supported input formats:

| Extension         | Format                             |
|-------------------|------------------------------------|
| `.epub`           | EPUB 2/3                           |
| `.mobi`           | MOBI (Amazon Kindle)               |
| `.pdf`            | PDF                                |
| `.md`/`.markdown` | Markdown file or directory project |

The format is chosen by the input extension, case-insensitively. Markdown accepts either a single `.md` file or a directory containing a `honzo.json` project config plus one or more `.md` files.

| Option              | Description       |
|---------------------|-------------------|
| `--title <text>`    | Override title    |
| `--author <text>`   | Override author   |
| `--language <code>` | Override language |

:::

::: card "**honzo-cli validate**" icon:check-circle
Validate a Honzo file's structure and integrity.

```bash
honzo-cli validate <file.hzo>
```

Checks:

- Magic bytes and format version
- Section offset consistency
- TOC entry integrity
- Chunk data reachability
- Compression round-trip

Returns exit code ::: tag "0" color:#22c55e if valid, ::: tag "1" color:#ef4444 otherwise.
:::

## Exit codes

| Code | Meaning       |
|------|---------------|
| `0`  | Success       |
| `1`  | General error |
| `2`  | Invalid input |
| `3`  | I/O error     |
