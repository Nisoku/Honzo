import { decode as decodeMsgPack } from "@msgpack/msgpack";
import { marked } from "marked";
import init, {
  HonzoWasm,
  normalize_search_term as normalizeSearchTerm,
} from "../wasm/honzo_wasm.js";
import { showLoading, hideLoading, showError, showToast } from "./ui.js";
import {
  bookTitle,
  chapterLabel,
  closeSidebar,
  currentChapter,
  currentChunkId,
  currentDirection,
  gapless,
  hasBook,
  layoutMode,
  pageZoom,
  referencePage,
  resetReaderState,
  textAlign,
  totalChapters,
  setCurrentBookId,
  setProgress,
  getProgress,
  getCurrentBookId,
  getBookmarks,
  addBookmark,
  removeBookmark,
  toggleSidebar,
} from "./state.js";
import "../satori.js";
import { renderBookmarks } from "./bookmarks.js";
import { renderMath } from "./math.js";
import { esc } from "../shared/esc.js";

let reader = null;
let meta = null;
let chapters = [];
let tocEntries = [];
let searchIndex = null;
let imageBlobs = [];
let currentChapterIndex = 0;
let pmapEntries = [];
let mangaPages = [];
let wasmInitPromise = null;
let elements = {
  currentPageInput: null,
  searchInput: null,
  searchResults: null,
  searchStatus: null,
  tocContent: null,
  viewer: null,
  chapterLabel: null,
  progressFill: null,
  footer: null,
  header: null,
  bookmarksContent: null,
  metaContent: null,
};

export function setBookElements(nextElements) {
  elements = { ...elements, ...nextElements };
}

function setSearchStatus(msg) {
  if (elements.searchStatus) elements.searchStatus.textContent = msg;
}

function clearSearchResults() {
  if (elements.searchResults) elements.searchResults.innerHTML = "";
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
    const excerpt = result.excerpt
      ? `<span class="search-excerpt">${esc(result.excerpt)}</span>`
      : "";
    item.innerHTML = `<span class="search-result-title">${esc(chapterLabel)}</span>${excerpt}`;
    item.addEventListener("click", () => {
      goToChapter(result.chapter.index);
      toggleSidebar(null);
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
    if (!bucket) return [];
    for (const [chunkId, offset] of bucket) {
      const key = `${chunkId}:${offset}`;
      if (!hitsByChunk.has(key)) {
        const entry = hitsByChunk.get(chunkId) || { score: 0 };
        entry.score += 1;
        hitsByChunk.set(chunkId, entry);
      }
    }
  }

  const matches = [];
  for (const [chunkId, entry] of hitsByChunk.entries()) {
    if (entry.score !== terms.length) continue;
    const ch = chapters.find((item) => item.chunk_id === chunkId);
    if (!ch) continue;
    const chapterText = reader.get_chapter_text(ch.index) || "";
    const encodedText = new TextEncoder().encode(chapterText);
    const offsets = [];
    for (const term of terms) {
      const bucket = getIndexBucket(term);
      if (!bucket) continue;
      for (const [cid, offset] of bucket) {
        if (cid === chunkId && !offsets.includes(offset)) offsets.push(offset);
      }
    }
    const firstOffset = offsets[0] || 0;
    const start = Math.max(0, firstOffset - 24);
    const end = Math.min(encodedText.length, firstOffset + 72);
    const excerpt = new TextDecoder()
      .decode(encodedText.slice(start, end))
      .replace(/\s+/g, " ")
      .trim();
    matches.push({ chapter: ch, score: entry.score, excerpt });
  }

  matches.sort(
    (a, b) => b.score - a.score || a.chapter.index - b.chapter.index,
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

function updateReferencePage(chapter) {
  if (!pmapEntries || pmapEntries.length === 0) {
    referencePage.set("");
    return;
  }
  let refPage = "";
  for (const entry of pmapEntries) {
    if (entry.chunkId === chapter.chunk_id) {
      refPage = `p. ${entry.printPage}`;
      break;
    }
  }
  referencePage.set(refPage);
}

function getMetaStr(obj, field) {
  if (!obj) return null;
  const v = obj[field];
  if (v instanceof Map) return v.values().next().value || null;
  if (typeof v === "object" && v !== null) return Object.values(v)[0] || null;
  return v || null;
}

function formatError(err) {
  if (err instanceof Error && err.message) return err.message;
  if (typeof err === "string") return err;
  return String(err);
}

async function ensureWasmReady() {
  if (!wasmInitPromise) wasmInitPromise = init();
  await wasmInitPromise;
}

export async function openBook(e) {
  const file = e.target.files[0];
  if (!file) return;
  showLoading();
  try {
    const data = await file.arrayBuffer();
    await loadBook(new Uint8Array(data), file.name);
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
    const name = bookPath.split("/").pop() || "book.hzo";
    await loadBook(new Uint8Array(data), name);
  } catch (err) {
    showError("Error loading demo: " + formatError(err));
  } finally {
    hideLoading();
  }
}

export async function openBookFromEntry(entry) {
  toggleSidebar(null);
  showLoading();
  try {
    const file =
      typeof entry.getFile === "function" ? await entry.getFile() : entry;
    const data = await file.arrayBuffer();
    await loadBook(new Uint8Array(data), file.name);
  } catch (err) {
    showError("Error opening book: " + formatError(err));
  } finally {
    hideLoading();
  }
}

// eslint-disable-next-line no-unused-vars
async function loadBook(data, fileName) {
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
  if (elements.footer) elements.footer.hidden = false;

  reader = new HonzoWasm(data, 1);
  meta = reader.get_meta_parsed();
  tocEntries = reader.get_toc();
  pmapEntries = reader.get_pmap() || [];
  mangaPages = tocEntries.filter((e) => e.chunk_type !== "SIDX");
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

  for (const e of tocEntries) {
    if (e.chunk_type === "CSS_") {
      const css = new TextDecoder().decode(reader.get_chunk(e.chunk_id));
      const style = document.createElement("style");
      style.textContent = css;
      document.head.appendChild(style);
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

  if (chapters.length === 0)
    throw new Error("This file has no readable content.");

  const titleVal = getMetaStr(meta, "title") || "Untitled";
  const bookId = `${titleVal}_${data.length}`;
  setCurrentBookId(bookId);

  currentChapterIndex = 0;
  if (layoutMode.get() === "manga") {
    renderManga();
    generateMangaToc();
  } else {
    renderCurrentChapter();
    generateToc();
    totalChapters.set(chapters.length);
  }
  renderMetaInfo();

  hasBook.set(true);

  window.removeEventListener("keyup", handleKeyEvents);
  window.addEventListener("keyup", handleKeyEvents);

  bookTitle.set(titleVal);

  const direction = meta?.direction || "ltr";
  currentDirection.set(direction);
  if (direction === "rtl") {
    textAlign.set("rtl");
  }

  const progress = getProgress(bookId);
  if (progress && progress.chapter >= 0 && progress.chapter < chapters.length) {
    goToChapter(progress.chapter);
  }

  if (elements.bookmarksContent) {
    renderBookmarks(elements.bookmarksContent, bookId, chapters);
  }

  if (window._layoutSub) window._layoutSub();
  window._layoutSub = layoutMode.subscribe(() => {
    if (layoutMode.get() === "manga") {
      renderManga();
      generateMangaToc();
    } else {
      renderCurrentChapter();
      generateToc();
    }
  });
  if (window._gaplessSub) window._gaplessSub();
  window._gaplessSub = gapless.subscribe(() => {
    const v = gapless.get();
    if (elements.viewer) {
      elements.viewer.classList.toggle("gapless", v);
    }
  });
  if (window._zoomSub) window._zoomSub();
  window._zoomSub = pageZoom.subscribe(() => {
    const v = pageZoom.get();
    if (layoutMode.get() === "manga") setMangaZoom(v);
  });
}

export function unloadBook() {
  if (window._layoutSub) {
    window._layoutSub();
    window._layoutSub = null;
  }
  if (window._gaplessSub) {
    window._gaplessSub();
    window._gaplessSub = null;
  }
  if (window._zoomSub) {
    window._zoomSub();
    window._zoomSub = null;
  }
  if (window._mangaScrollHandler && elements.viewer) {
    elements.viewer.removeEventListener("scroll", window._mangaScrollHandler);
    window._mangaScrollHandler = null;
  }
  mangaPages = [];
  currentChapterIndex = 0;
  closeSidebar(null);
  resetReaderState();
}

function renderManga() {
  if (!mangaPages.length || !elements.viewer) return;
  elements.viewer.innerHTML = "";
  elements.viewer.classList.add("manga-mode");

  const container = document.createElement("div");
  container.className = "manga-container";

  for (const entry of mangaPages) {
    const page = document.createElement("div");
    page.className = "manga-page";
    page.dataset.chunkId = entry.chunk_id;
    page.dataset.chunkIndex = mangaPages.indexOf(entry);

    const chunkType = entry.chunk_type;
    const data = reader.get_chunk(entry.chunk_id);

    if (chunkType === "IMG_" || chunkType === "COVR" || chunkType === "COVT") {
      const mime = guessMime(data);
      const blob = new Blob([data], { type: mime });
      const url = URL.createObjectURL(blob);
      const img = document.createElement("img");
      img.src = url;
      img.alt = `Page ${entry.chunk_id}`;
      page.appendChild(img);
    } else if (chunkType === "CHAP" || chunkType === "NOTE") {
      const raw = new TextDecoder().decode(data);
      const isHtml =
        entry.content_type_kind === 1 && entry.content_type_value === 1;
      const content = isHtml ? sanitizeHtml(raw) : renderMarkdown(raw);
      const body = document.createElement("div");
      body.className = "manga-text chapter-body";
      body.innerHTML = content;
      page.appendChild(body);
    } else if (chunkType === "MATH") {
      const mathDiv = document.createElement("div");
      mathDiv.className = "manga-math";
      renderMath(mathDiv, data, entry.content_type_value);
      page.appendChild(mathDiv);
    } else {
      const label = document.createElement("p");
      label.className = "manga-placeholder";
      label.textContent = `${chunkType} #${entry.chunk_id}`;
      page.appendChild(label);
    }

    container.appendChild(page);
  }

  elements.viewer.appendChild(container);

  if (gapless.get()) {
    elements.viewer.classList.add("gapless");
  }

  if (window._mangaScrollHandler) {
    elements.viewer.removeEventListener("scroll", window._mangaScrollHandler);
  }
  window._mangaScrollHandler = () => {
    updateMangaPageInfo();
  };
  elements.viewer.addEventListener("scroll", window._mangaScrollHandler, {
    passive: true,
  });

  setMangaZoom(pageZoom.get());
  updateMangaPageInfo();
}

function scrollToMangaPage(index) {
  if (!elements.viewer || !mangaPages.length) return;
  const clamped = Math.max(0, Math.min(index, mangaPages.length - 1));
  const page = elements.viewer.querySelector(
    `.manga-page[data-chunk-index="${clamped}"]`,
  );
  if (page) {
    page.scrollIntoView({ behavior: "smooth", block: "start" });
  }
  updateMangaPageInfo();
}

function updateMangaPageInfo() {
  if (!mangaPages.length || !elements.viewer) return;
  const idx = getMangaCurrentPage();
  const entry = mangaPages[idx];
  if (!entry) return;

  currentChapter.set(idx);
  currentChunkId.set(entry.chunk_id);

  totalChapters.set(mangaPages.length);
  hasBook.set(true);

  const label = `${entry.chunk_type} ${entry.chunk_id}${entry.size_raw ? ` (${(entry.size_raw / 1024).toFixed(0)} KB)` : ""}`;
  chapterLabel.set(label);

  updateReferencePage(entry);

  const bookId = getCurrentBookId();
  if (bookId) setProgress(bookId, idx, entry.chunk_id);

  if (elements.progressFill) {
    elements.progressFill.style.width = `${((idx + 1) / mangaPages.length) * 100}%`;
  }

  if (elements.tocContent) {
    elements.tocContent.querySelectorAll(".toc-item").forEach((item, i) => {
      item.classList.toggle("active", i === idx);
    });
  }
}

function getMangaCurrentPage() {
  if (!elements.viewer || !mangaPages.length) return 0;
  const pages = elements.viewer.querySelectorAll(".manga-page");
  let closest = 0;
  let closestDist = Infinity;
  pages.forEach((p, i) => {
    const rect = p.getBoundingClientRect();
    const pageCenter = rect.top + rect.height / 2;
    const dist = Math.abs(pageCenter - elements.viewer.clientHeight / 2);
    if (dist < closestDist) {
      closestDist = dist;
      closest = i;
    }
  });
  return closest;
}

function generateMangaToc() {
  if (!mangaPages.length || !elements.tocContent) return;
  elements.tocContent.innerHTML = "";
  mangaPages.forEach((entry, i) => {
    const isCurrent = i === getMangaCurrentPage();
    const item = document.createElement("div");
    item.className = `toc-item${isCurrent ? " active" : ""}`;
    const label = `${entry.chunk_type} #${entry.chunk_id}${entry.chunk_type === "CHAP" || entry.chunk_type === "IMG_" ? "" : ""}`;
    const sizeInfo = entry.size_raw
      ? `${(entry.size_raw / 1024).toFixed(0)} KB`
      : "";
    item.innerHTML = `<span class="toc-label">${esc(label)}</span>${sizeInfo ? `<span class="toc-size">${esc(sizeInfo)}</span>` : ""}`;
    item.addEventListener("click", () => {
      scrollToMangaPage(i);
      toggleSidebar(null);
    });
    elements.tocContent.appendChild(item);
  });
}

function renderCurrentChapter(scrollToAnchor) {
  if (layoutMode.get() === "manga") {
    renderManga();
    return;
  }
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
    renderMath(container, data, chapter.content_type_value);
  } else if (
    chapter.chunk_type === "SIDX" ||
    chapter.chunk_type === "CSS_" ||
    chapter.chunk_type === "FONT"
  ) {
    container.innerHTML = '<p class="meta">No preview available</p>';
  } else {
    const raw = new TextDecoder().decode(data);
    const isHtml =
      chapter.content_type_kind === 1 && chapter.content_type_value === 1;

    let content;
    if (isHtml) {
      content = sanitizeHtml(raw);
    } else {
      content = renderMarkdown(raw);
    }

    container.innerHTML = `<div class="chapter-body">${content}</div>`;
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
          goToChapter(chapterIndex, anchor || undefined);
        });
        ref.replaceWith(link);
      }
    }
  }

  elements.viewer.appendChild(container);

  if (scrollToAnchor) {
    const target = container.querySelector(`#${CSS.escape(scrollToAnchor)}`);
    if (target) target.scrollIntoView({ behavior: "smooth", block: "center" });
  }

  if (!scrollToAnchor) {
    elements.viewer.scrollTop = 0;
  }

  currentChapter.set(currentChapterIndex);
  currentChunkId.set(chapter.chunk_id);

  updateReferencePage(chapter);

  const label = `${chapter.chunk_type} ${chapter.chunk_id}${chapter.size_raw ? ` (${(chapter.size_raw / 1024).toFixed(0)} KB)` : ""}`;
  chapterLabel.set(label);

  if (elements.progressFill) {
    const t = chapters.length;
    elements.progressFill.style.width = `${t > 1 ? ((currentChapterIndex + 1) / t) * 100 : 0}%`;
  }

  const bookId = getCurrentBookId();
  if (bookId) {
    setProgress(bookId, currentChapterIndex, chapter.chunk_id);
  }
}

export function goToChapter(index, anchor) {
  if (!chapters.length) return;
  if (layoutMode.get() === "manga") {
    scrollToMangaPage(index);
    return;
  }
  currentChapterIndex = clampChapterIndex(index);
  renderCurrentChapter(anchor);
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

function generateToc() {
  if (!chapters.length || !elements.tocContent) return;
  elements.tocContent.innerHTML = "";
  chapters.forEach((ch, i) => {
    const isCurrent = i === currentChapterIndex;
    const item = document.createElement("div");
    item.className = `toc-item${isCurrent ? " active" : ""}`;
    const label =
      ch.chunk_type === "CHAP"
        ? `Chapter ${ch.chunk_id}`
        : `${ch.chunk_type} ${ch.chunk_id}`;
    const sizeInfo = ch.size_raw ? `${(ch.size_raw / 1024).toFixed(0)} KB` : "";
    item.innerHTML = `<span class="toc-label">${esc(label)}</span>${sizeInfo ? `<span class="toc-size">${esc(sizeInfo)}</span>` : ""}`;
    item.addEventListener("click", () => {
      goToChapter(i);
      toggleSidebar(null);
    });
    elements.tocContent.appendChild(item);
  });
}

function renderMetaInfo() {
  if (!elements.metaContent) return;
  const m = meta;
  if (!m || typeof m !== "object") {
    elements.metaContent.innerHTML =
      "<p class='meta-empty'>No metadata available</p>";
    return;
  }

  const title = getMetaStr(m, "title") || "Untitled";
  const author =
    getMetaStr(m, "authors") || getMetaStr(m, "author") || "Unknown";
  const publisher = getMetaStr(m, "publisher");
  const description = getMetaStr(m, "description");
  const language = getMetaStr(m, "language");
  const wordCount = m.word_count;
  const readingTime = m.reading_time_mins;
  const edition = getMetaStr(m, "edition");
  const source = getMetaStr(m, "source_url");
  const license = getMetaStr(m, "license");
  const genres = Array.isArray(m.genres) ? m.genres : [];
  const tags = Array.isArray(m.tags) ? m.tags : [];

  const series = m.series;
  const identifiers = Array.isArray(m.identifiers) ? m.identifiers : [];

  let html = `
    <div class="meta-section">
      <h3 class="meta-title">${esc(title)}</h3>
      <p class="meta-author">${esc(author)}</p>
      ${publisher ? `<p class="meta-publisher">${esc(publisher)}</p>` : ""}
    </div>
  `;

  if (description) {
    html += `<div class="meta-section"><p class="meta-description">${esc(description)}</p></div>`;
  }

  html += `<div class="meta-grid">`;
  if (language) html += metaGridItem("Language", language);
  if (wordCount !== null && wordCount !== undefined)
    html += metaGridItem("Words", Number(wordCount).toLocaleString());
  if (readingTime !== null && readingTime !== undefined)
    html += metaGridItem("Reading Time", `${readingTime} min`);
  if (edition) html += metaGridItem("Edition", edition);
  if (source)
    html += metaGridItem(
      "Source",
      `<a href="${esc(source)}" target="_blank" rel="noopener">${esc(new URL(source).hostname)}</a>`,
    );
  if (license) html += metaGridItem("License", license);
  html += metaGridItem("Chapters", String(chapters.length));
  html += metaGridItem("File Type", ".hzo");
  html += `</div>`;

  if (series) {
    html += `<div class="meta-section"><h4>Series</h4>`;
    html += `<p>${esc(series.title || "")}${series.position ? ` · ${esc(series.position)}` : ""}${series.arc ? ` · ${esc(series.arc)}` : ""}</p>`;
    html += `</div>`;
  }

  if (genres.length) {
    html += `<div class="meta-section"><h4>Genres</h4><div class="meta-tags">${genres.map((g) => `<span class="meta-tag">${esc(g)}</span>`).join("")}</div></div>`;
  }

  if (tags.length) {
    html += `<div class="meta-section"><h4>Tags</h4><div class="meta-tags">${tags.map((t) => `<span class="meta-tag">${esc(t)}</span>`).join("")}</div></div>`;
  }

  if (identifiers.length) {
    html += `<div class="meta-section"><h4>Identifiers</h4><div class="meta-ids">${identifiers.map((id) => `<div class="meta-id"><span class="meta-id-type">${esc(id.id_type)}</span><span class="meta-id-value">${esc(id.value)}</span></div>`).join("")}</div></div>`;
  }

  elements.metaContent.innerHTML = html;
}

function metaGridItem(label, value) {
  return `<div class="meta-grid-item"><span class="meta-grid-label">${label}</span><span class="meta-grid-value">${value}</span></div>`;
}

function hasBookLoaded() {
  return chapters.length > 0;
}

function clampChapterIndex(index) {
  if (!chapters.length) return 0;
  return Math.max(0, Math.min(index, chapters.length - 1));
}

export function prevPage() {
  if (!hasBookLoaded()) return;
  if (layoutMode.get() === "manga") {
    if (!elements.viewer) return;
    elements.viewer.scrollBy({
      top: -elements.viewer.clientHeight,
      behavior: "smooth",
    });
    return;
  }
  if (currentChapterIndex <= 0) {
    showToast("First page");
    return;
  }
  goToChapter(currentChapterIndex - 1);
}

export function nextPage() {
  if (!hasBookLoaded()) return;
  if (layoutMode.get() === "manga") {
    if (!elements.viewer) return;
    elements.viewer.scrollBy({
      top: elements.viewer.clientHeight,
      behavior: "smooth",
    });
    return;
  }
  if (currentChapterIndex >= chapters.length - 1) {
    showToast("Last page");
    return;
  }
  goToChapter(currentChapterIndex + 1);
}

export function addBookmarkCurrent() {
  const bookId = getCurrentBookId();
  if (!bookId || !hasBookLoaded()) return;
  addBookmark(bookId, currentChapterIndex, currentChunkId.value, "");
  if (elements.bookmarksContent) {
    renderBookmarks(elements.bookmarksContent, bookId, chapters);
  }
}

export function removeBookmarkCurrent() {
  const bookId = getCurrentBookId();
  if (!bookId || !hasBookLoaded()) return;
  const marks = getBookmarks(bookId);
  const idx = marks.findIndex((m) => m.chapter === currentChapterIndex);
  if (idx >= 0) {
    removeBookmark(bookId, idx);
    if (elements.bookmarksContent) {
      renderBookmarks(elements.bookmarksContent, bookId, chapters);
    }
  }
}

export function isCurrentChapterBookmarked() {
  const bookId = getCurrentBookId();
  if (!bookId) return false;
  return getBookmarks(bookId).some((m) => m.chapter === currentChapterIndex);
}

function handleKeyEvents(e) {
  if (!hasBookLoaded()) return;
  if (e.key === "ArrowLeft") prevPage();
  if (e.key === "ArrowRight") nextPage();
  if (e.key === "b" && (e.metaKey || e.ctrlKey)) {
    e.preventDefault();
    if (isCurrentChapterBookmarked()) {
      removeBookmarkCurrent();
    } else {
      addBookmarkCurrent();
    }
  }
  if (e.key === "f" && (e.metaKey || e.ctrlKey)) {
    e.preventDefault();
    toggleSidebar("search");
    if (elements.searchInput)
      setTimeout(() => elements.searchInput.focus(), 100);
  }
  if (layoutMode.get() === "manga") {
    if (e.key === "=" || e.key === "+") {
      e.preventDefault();
      const v = Math.min(2.5, +(pageZoom.get() + 0.1).toFixed(1));
      pageZoom.set(v);
      setMangaZoom(v);
    }
    if (e.key === "-" || e.key === "_") {
      e.preventDefault();
      const v = Math.max(0.5, +(pageZoom.get() - 0.1).toFixed(1));
      pageZoom.set(v);
      setMangaZoom(v);
    }
    if (e.key === "0" && !e.metaKey && !e.ctrlKey) {
      e.preventDefault();
      pageZoom.set(1);
      setMangaZoom(1);
    }
  }
}

function guessMime(bytes) {
  if (bytes.length < 4) return "image/jpeg";
  const b = (i) => bytes[i];
  if (b(0) === 0x89 && b(1) === 0x50 && b(2) === 0x4e && b(3) === 0x47)
    return "image/png";
  if (b(0) === 0xff && b(1) === 0xd8) return "image/jpeg";
  if (b(0) === 0x47 && b(1) === 0x49 && b(2) === 0x46) return "image/gif";
  if (b(0) === 0x52 && b(1) === 0x49 && b(2) === 0x46 && b(3) === 0x46)
    return "image/webp";
  if (b(0) === 0x3c) {
    const head = new TextDecoder().decode(bytes.slice(0, 512));
    if (head.includes("<svg")) return "image/svg+xml";
  }
  return "image/jpeg";
}

export function setMangaZoom(zoom) {
  if (!elements.viewer) return;
  elements.viewer.querySelectorAll(".manga-page img").forEach((img) => {
    img.style.width = zoom * 100 + "%";
  });
}

export function toggleToc() {
  toggleSidebar("toc");
}

export function closeToc() {
  toggleSidebar(null);
}

export function focusSearch() {
  elements.searchInput?.focus();
  elements.searchInput?.select();
}
