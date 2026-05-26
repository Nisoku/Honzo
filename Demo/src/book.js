import { decode as decodeMsgPack } from "@msgpack/msgpack";
import { marked } from "marked";
import init, {
  HonzoWasm,
  normalize_search_term as normalizeSearchTerm,
  render_math as renderMath,
} from "./wasm/honzo_wasm.js";
import {
  showLoading,
  hideLoading,
  showError,
  toggleLibrary,
  toggleToc as setTocOpen,
} from "./ui.js";
import {
  bookTitle,
  currentPage,
  hasBook,
  resetReaderState,
  totalPages,
} from "./state.js";

let reader = null;
let meta = null;
let chapters = [];
let tocEntries = [];
let searchIndex = null;
let imageBlobs = [];
let currentChapterIndex = 0;
let wasmInitPromise = null;
let elements = {
  currentPageInput: null,
  searchInput: null,
  searchResults: null,
  searchStatus: null,
  tocContent: null,
  viewer: null,
};

export function setBookElements(nextElements) {
  elements = {
    ...elements,
    ...nextElements,
  };
}

function setSearchStatus(message) {
  if (elements.searchStatus) {
    elements.searchStatus.textContent = message;
  }
}

function clearSearchResults() {
  if (elements.searchResults) {
    elements.searchResults.innerHTML = "";
  }
}

function setSearchResults(results) {
  if (!elements.searchResults) return;
  elements.searchResults.innerHTML = "";

  if (!searchIndex) {
    setSearchStatus("This book does not include a search index.");
    return;
  }

  if (results.length === 0) {
    setSearchStatus("No matches.");
    return;
  }

  setSearchStatus(`${results.length} result${results.length === 1 ? "" : "s"}`);

  for (const result of results) {
    const item = document.createElement("button");
    item.type = "button";
    item.className = "search-result";
    const chapterLabel = `${result.chapter.chunk_type} #${result.chapter.chunk_id}`;
    const summary = `${chapterLabel} • ${result.score} match${result.score === 1 ? "" : "es"}`;
    const excerpt = result.excerpt ? `<span>${esc(result.excerpt)}</span>` : "";
    item.innerHTML = `<strong>${esc(summary)}</strong>${excerpt}`;
    item.addEventListener("click", () => {
      goToChapter(result.chapter.index);
      setTocOpen(false);
    });
    elements.searchResults.appendChild(item);
  }
}

function parseSearchIndex(bytes) {
  if (!bytes || bytes.length === 0) return null;
  const decoded = decodeMsgPack(bytes);
  if (decoded instanceof Map) return decoded;
  if (decoded && typeof decoded === "object")
    return new Map(Object.entries(decoded));
  return null;
}

function getIndexBucket(term) {
  if (!searchIndex) return null;
  if (searchIndex instanceof Map) return searchIndex.get(term) || null;
  return searchIndex[term] || null;
}

function searchChapters(query) {
  if (!searchIndex) return [];
  const terms = query
    .trim()
    .split(/\s+/)
    .map((term) => normalizeSearchTerm(term, meta?.language || "en"))
    .filter(Boolean);

  if (terms.length === 0) {
    setSearchStatus("Type a word to search.");
    return [];
  }

  const hitsByChunk = new Map();
  for (const term of terms) {
    const bucket = getIndexBucket(term);
    if (!bucket) {
      return [];
    }
    const seenOffsets = new Set();
    const seenChunks = new Set();
    for (const [chunkId, offset] of bucket) {
      const key = `${chunkId}:${offset}`;
      if (!seenOffsets.has(key)) {
        seenOffsets.add(key);
        const entry = hitsByChunk.get(chunkId) || { score: 0, offsets: [] };
        entry.offsets.push(offset);
        hitsByChunk.set(chunkId, entry);
      }
      if (!seenChunks.has(chunkId)) {
        seenChunks.add(chunkId);
        const entry = hitsByChunk.get(chunkId) || { score: 0, offsets: [] };
        entry.score += 1;
        hitsByChunk.set(chunkId, entry);
      }
    }
  }

  const matches = [];
  for (const [chunkId, entry] of hitsByChunk.entries()) {
    if (entry.score !== terms.length) continue;
    const chapter = chapters.find((item) => item.chunk_id === chunkId);
    if (!chapter) continue;
    const chapterText = reader.get_chapter_text(chapter.index) || "";
    const encodedText = new TextEncoder().encode(chapterText);
    const firstOffset = entry.offsets[0] || 0;
    const start = Math.max(0, firstOffset - 24);
    const end = Math.min(encodedText.length, firstOffset + 72);
    const excerpt = new TextDecoder()
      .decode(encodedText.slice(start, end))
      .replace(/\s+/g, " ")
      .trim();
    matches.push({ chapter, score: entry.score, excerpt });
  }

  matches.sort(
    (left, right) =>
      right.score - left.score || left.chapter.index - right.chapter.index,
  );
  return matches.slice(0, 25);
}

export function runSearch(query) {
  if (!elements.searchResults) return;
  if (!searchIndex) {
    clearSearchResults();
    setSearchStatus("This book does not include a search index.");
    return;
  }
  if (!query.trim()) {
    clearSearchResults();
    setSearchStatus("Type a word to search.");
    return;
  }
  setSearchResults(searchChapters(query));
}

function getMetaStr(obj, field) {
  if (!obj) return null;
  const v = obj[field];
  if (v instanceof Map) return v.values().next().value || null;
  if (typeof v === "object" && v !== null) return Object.values(v)[0] || null;
  return v || null;
}

function formatError(err) {
  if (err instanceof Error && err.message) {
    return err.message;
  }
  if (typeof err === "string") {
    return err;
  }
  return String(err);
}

async function ensureWasmReady() {
  if (!wasmInitPromise) {
    wasmInitPromise = init();
  }
  await wasmInitPromise;
}

export async function openBook(e) {
  const file = e.target.files[0];
  if (!file) return;
  showLoading();
  try {
    const data = await file.arrayBuffer();
    await loadBook(new Uint8Array(data));
  } catch (err) {
    showError("Error loading book: " + formatError(err));
  } finally {
    hideLoading();
  }
}

export async function openBuiltinBook(bookPath) {
  if (!bookPath) return;
  showLoading();
  try {
    const resp = await fetch(bookPath);
    if (!resp.ok) throw new Error("HTTP " + resp.status);
    const data = await resp.arrayBuffer();
    await loadBook(new Uint8Array(data));
  } catch (err) {
    showError("Error loading demo: " + formatError(err));
  } finally {
    hideLoading();
  }
}

export async function openBookFromEntry(entry) {
  toggleLibrary(false);
  showLoading();
  try {
    const file =
      typeof entry.getFile === "function" ? await entry.getFile() : entry;
    const data = await file.arrayBuffer();
    await loadBook(new Uint8Array(data));
  } catch (err) {
    toggleLibrary(true);
    showError("Error opening book: " + formatError(err));
  } finally {
    hideLoading();
  }
}

async function loadBook(data) {
  await ensureWasmReady();
  if (!elements.viewer || !elements.tocContent) {
    throw new Error("Reader UI is not ready.");
  }

  resetReaderState();
  elements.viewer.innerHTML = "";
  elements.tocContent.innerHTML = "";
  if (elements.searchInput) elements.searchInput.value = "";
  clearSearchResults();
  setSearchStatus("Type a word to search.");
  reader = new HonzoWasm(data, 1);
  meta = reader.get_meta_parsed();
  tocEntries = reader.get_toc();
  const sidxEntry = tocEntries.find((entry) => entry.chunk_type === "SIDX");
  searchIndex = null;
  if (sidxEntry) {
    try {
      searchIndex = parseSearchIndex(reader.get_chunk(sidxEntry.chunk_id));
    } catch {
      searchIndex = null;
    }
  }

  imageBlobs = new Map();
  for (const e of tocEntries) {
    if (e.chunk_type === "IMG_" || e.chunk_type === "COVR") {
      const data = reader.get_chunk(e.chunk_id);
      const mime = guessMime(data);
      const blob = new Blob([data], { type: mime });
      imageBlobs.set(e.chunk_id, { blob, url: URL.createObjectURL(blob) });
    }
  }

  chapters = tocEntries
    .filter(
      (e) =>
        e.chunk_type === "CHAP" ||
        e.chunk_type === "NOTE" ||
        e.chunk_type === "MATH",
    )
    .map((e, i) => ({ index: i, chunk_id: e.chunk_id, ...e }));

  if (chapters.length === 0) {
    chapters = tocEntries.map((e, i) => ({
      index: i,
      chunk_id: e.chunk_id,
      ...e,
    }));
  }

  if (chapters.length === 0) {
    throw new Error("This file has no readable content.");
  }

  currentChapterIndex = 0;
  renderCurrentChapter();
  generateToc();

  totalPages.set(chapters.length);
  currentPage.set(1);
  hasBook.set(true);

  window.removeEventListener("keyup", handleKeyEvents);
  window.addEventListener("keyup", handleKeyEvents);

  const titleVal = getMetaStr(meta, "title");
  bookTitle.set(titleVal || "Untitled");
}

// TODO: Probably build a viewless renderer so that we don't have to handle the <ref> stuff 
// TODO: and it's cleaner and safer
function renderCurrentChapter() {
  if (!chapters.length) return;
  if (!elements.viewer) return;
  const chapter = chapters[currentChapterIndex];
  const data = reader.get_chunk(chapter.chunk_id);

  elements.viewer.innerHTML = "";
  const container = document.createElement("article");
  container.className = "chapter-view";

  const isImage =
    chapter.chunk_type === "IMG_" ||
    chapter.chunk_type === "COVR" ||
    chapter.chunk_type === "COVT";
  const isMath = chapter.chunk_type === "MATH";

  if (isImage) {
    const blob = new Blob([data], { type: "image/jpeg" });
    const url = URL.createObjectURL(blob);
    const img = document.createElement("img");
    img.src = url;
    container.appendChild(img);
  } else if (isMath) {
    const raw = new TextDecoder().decode(data);
    try {
      const rendered = renderMath(data, chapter.content_type_value);
      container.innerHTML = '<div class="math-block">' + rendered + "</div>";
    } catch {
      container.innerHTML = '<pre class="math-latex">' + esc(raw) + "</pre>";
    }
  } else if (
    chapter.chunk_type === "SIDX" ||
    chapter.chunk_type === "CSS_" ||
    chapter.chunk_type === "FONT"
  ) {
    container.innerHTML =
      '<p class="meta">[' +
      chapter.chunk_type +
      " chunk " +
      chapter.chunk_id +
      " - " +
      data.length +
      " bytes]</p>";
  } else {
    const raw = new TextDecoder().decode(data);
    const isHtml =
      chapter.content_type_kind === 1 && chapter.content_type_value === 1;

    if (isHtml) {
      container.innerHTML = sanitizeHtml(raw);
    } else {
      container.innerHTML = renderMarkdown(raw);
    }
  }

  for (const ref of container.querySelectorAll("ref")) {
    const type = ref.getAttribute("type");
    const chunkId = parseInt(ref.getAttribute("chunk"), 10);
    if (type === "image" && !isNaN(chunkId)) {
      const match = imageBlobs.get(chunkId);
      if (match) {
        const img = document.createElement("img");
        img.src = match.url;
        const alt = ref.getAttribute("alt");
        if (alt) img.alt = alt;
        ref.replaceWith(img);
      }
    } else if (type === "chapter" && !isNaN(chunkId)) {
      const chapterIndex = chapters.findIndex((ch) => ch.chunk_id === chunkId);
      if (chapterIndex !== -1) {
        const link = document.createElement("a");
        link.href = "#";
        link.className = "chapter-ref";
        const anchor = ref.getAttribute("anchor");
        link.textContent = anchor
          ? `Chapter #${chunkId} (${anchor})`
          : `Chapter #${chunkId}`;
        link.addEventListener("click", (e) => {
          e.preventDefault();
          goToChapter(chapterIndex);
        });
        ref.replaceWith(link);
      }
    }
  }

  elements.viewer.appendChild(container);
  currentPage.set(currentChapterIndex + 1);
}

function goToChapter(index) {
  if (!chapters.length) return;
  currentChapterIndex = clampChapterIndex(index);
  renderCurrentChapter();
}

function sanitizeHtml(html) {
  const allowedTags = [
    "p",
    "div",
    "span",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "ul",
    "ol",
    "li",
    "dl",
    "dt",
    "dd",
    "table",
    "thead",
    "tbody",
    "tr",
    "th",
    "td",
    "blockquote",
    "pre",
    "code",
    "br",
    "hr",
    "em",
    "strong",
    "i",
    "b",
    "u",
    "s",
    "sub",
    "sup",
    "img",
    "a",
    "figure",
    "figcaption",
    "section",
    "article",
    "header",
    "main",
    "aside",
    "ref",
    // MathML
    "math",
    "mi",
    "mo",
    "mn",
    "msup",
    "msub",
    "mfrac",
    "msqrt",
    "mroot",
    "mstyle",
    "mrow",
    "mspace",
    "mtext",
    "munder",
    "mover",
    "munderover",
    "msubsup",
    "mmultiscripts",
    "mprescripts",
    "none",
    "mtable",
    "mtr",
    "mtd",
    "mphantom",
    "mfenced",
    "menclose",
    "merror",
    "mpadded",
    "maction",
    "mlabeledtr",
    "maligngroup",
    "malignmark",
    "msline",
  ];
  const allowedAttrs = [
    "href",
    "src",
    "alt",
    "title",
    "class",
    "id",
    "name",
    "target",
    "rel",
    "width",
    "height",
    "type",
    "chunk",
    "anchor",
    "xmlns",
    "display",
    "alttext",
    "rowspan",
    "columnspan",
    "linethickness",
    "lspace",
    "voffset",
    "scriptlevel",
    "displaystyle",
  ];
  const doc = new DOMParser().parseFromString(html, "text/html");
  const walk = (node) => {
    if (node.nodeType === 3) return node;
    if (node.nodeType !== 1) {
      node.remove();
      return null;
    }
    const tag = node.tagName.toLowerCase();
    if (!allowedTags.includes(tag)) {
      const span = document.createElement("span");
      while (node.firstChild) span.appendChild(node.firstChild);
      node.replaceWith(span);
      return span;
    }
    const keep = {};
    for (const attr of node.attributes) {
      if (allowedAttrs.includes(attr.name)) keep[attr.name] = attr.value;
    }
    while (node.attributes.length > 0)
      node.removeAttribute(node.attributes[0].name);
    for (const [k, v] of Object.entries(keep)) node.setAttribute(k, v);
    if (tag === "a" && node.getAttribute("href")?.startsWith("http")) {
      node.setAttribute("target", "_blank");
      node.setAttribute("rel", "noopener");
    }
    Array.from(node.childNodes).forEach((c) => walk(c));
    return node;
  };
  Array.from(doc.body.childNodes).forEach((c) => walk(c));
  return doc.body.innerHTML;
}

function renderMarkdown(text) {
  return marked.parse(text);
}

function esc(s) {
  return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

function generateToc() {
  if (!chapters.length || !elements.tocContent) return;
  elements.tocContent.innerHTML = "";
  chapters.forEach((ch, i) => {
    const item = document.createElement("div");
    item.className = "toc-item";
    const label =
      ch.chunk_type + " #" + ch.chunk_id + " (" + ch.size_raw + " bytes)";
    item.textContent = label;
    item.addEventListener("click", () => {
      currentChapterIndex = i;
      renderCurrentChapter();
      closeToc();
    });
    elements.tocContent.appendChild(item);
  });
}

function hasBookLoaded() {
  return chapters.length > 0;
}

function clampChapterIndex(index) {
  if (!chapters.length) return 0;
  return Math.max(0, Math.min(index, chapters.length - 1));
}

function normalizedPageNumber() {
  const p = parseInt(elements.currentPageInput?.value ?? "", 10);
  return Number.isNaN(p) ? currentChapterIndex + 1 : p;
}

export function prevPage() {
  if (!hasBookLoaded() || currentChapterIndex <= 0) return;
  goToChapter(currentChapterIndex - 1);
}

export function nextPage() {
  if (!hasBookLoaded() || currentChapterIndex >= chapters.length - 1) return;
  goToChapter(currentChapterIndex + 1);
}

export function goToPage() {
  if (!hasBookLoaded()) return;
  goToChapter(normalizedPageNumber() - 1);
}

function handleKeyEvents(e) {
  if (!hasBookLoaded()) return;
  if (e.key === "ArrowLeft") prevPage();
  if (e.key === "ArrowRight") nextPage();
}

function guessMime(bytes) {
  if (bytes.length < 4) return "image/jpeg";
  const b = (i) => bytes[i];
  if (b(0) === 0x89 && b(1) === 0x50 && b(2) === 0x4e && b(3) === 0x47) return "image/png";
  if (b(0) === 0xff && b(1) === 0xd8) return "image/jpeg";
  if (b(0) === 0x47 && b(1) === 0x49 && b(2) === 0x46) return "image/gif";
  if (b(0) === 0x52 && b(1) === 0x49 && b(2) === 0x46 && b(3) === 0x46) return "image/webp";
  if (b(0) === 0x3c) {
    const head = new TextDecoder().decode(bytes.slice(0, 512));
    if (head.includes("<svg")) return "image/svg+xml";
  }
  return "image/jpeg";
}

export function toggleToc() {
  setTocOpen();
}

export function closeToc() {
  setTocOpen(false);
}

export function focusSearch() {
  elements.searchInput?.focus();
  elements.searchInput?.select();
}
