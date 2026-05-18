import init, { HonzoWasm } from "./wasm/honzo_wasm.js";
import { showLoading, hideLoading, showError, toggleLibrary, toggleToc as setTocOpen } from "./ui.js";
import {
  bookTitle,
  currentPage,
  hasBook,
  resetReaderState,
  tocOpen,
  totalPages,
} from "./state.js";

let reader = null;
let meta = null;
let chapters = [];
let tocEntries = [];
let currentChapterIndex = 0;
let wasmInitPromise = null;
let elements = {
  currentPageInput: null,
  tocContent: null,
  viewer: null,
};

export function setBookElements(nextElements) {
  elements = {
    ...elements,
    ...nextElements,
  };
}

function val(maybeMap) {
  if (!maybeMap) return null;
  if (maybeMap instanceof Map) return maybeMap.values().next().value || null;
  if (typeof maybeMap === 'object') return Object.values(maybeMap)[0] || null;
  return maybeMap;
}

function getMetaStr(obj, field) {
  if (!obj) return null;
  const v = obj[field];
  if (v instanceof Map) return v.values().next().value || null;
  if (typeof v === 'object' && v !== null) return Object.values(v)[0] || null;
  return v || null;
}

function formatError(err) {
  if (err instanceof Error && err.message) {
    return err.message;
  }
  if (typeof err === 'string') {
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
    showError('Error loading book: ' + formatError(err));
  } finally {
    hideLoading();
  }
}

export async function openBuiltinBook(bookPath) {
  if (!bookPath) return;
  showLoading();
  try {
    const resp = await fetch(bookPath);
    if (!resp.ok) throw new Error('HTTP ' + resp.status);
    const data = await resp.arrayBuffer();
    await loadBook(new Uint8Array(data));
  } catch (err) {
    showError('Error loading demo: ' + formatError(err));
  } finally {
    hideLoading();
  }
}

export async function openBookFromEntry(entry) {
  toggleLibrary(false);
  showLoading();
  try {
    const file = typeof entry.getFile === 'function' ? await entry.getFile() : entry;
    const data = await file.arrayBuffer();
    await loadBook(new Uint8Array(data));
  } catch (err) {
    toggleLibrary(true);
    showError('Error opening book: ' + formatError(err));
  } finally {
    hideLoading();
  }
}

async function loadBook(data) {
  await ensureWasmReady();
  if (!elements.viewer || !elements.tocContent) {
    throw new Error('Reader UI is not ready.');
  }

  resetReaderState();
  elements.viewer.innerHTML = '';
  elements.tocContent.innerHTML = '';
  reader = new HonzoWasm(data, 1);
  meta = reader.get_meta_parsed();
  tocEntries = reader.get_toc();

  chapters = tocEntries
    .filter(e => e.chunk_type === 'CHAP' || e.chunk_type === 'NOTE')
    .map((e, i) => ({ index: i, chunk_id: e.chunk_id, ...e }));

  if (chapters.length === 0) {
    chapters = tocEntries.map((e, i) => ({ index: i, chunk_id: e.chunk_id, ...e }));
  }

  if (chapters.length === 0) {
    throw new Error('This file has no readable content.');
  }

  currentChapterIndex = 0;
  renderCurrentChapter();
  generateToc();

  totalPages.set(chapters.length);
  currentPage.set(1);
  hasBook.set(true);

  window.removeEventListener('keyup', handleKeyEvents);
  window.addEventListener('keyup', handleKeyEvents);

  const titleVal = getMetaStr(meta, 'title');
  bookTitle.set(titleVal || 'Untitled');
}

function renderCurrentChapter() {
  if (!chapters.length) return;
  if (!elements.viewer) return;
  const chapter = chapters[currentChapterIndex];
  const data = reader.get_chunk(chapter.chunk_id);

  elements.viewer.innerHTML = '';
  const container = document.createElement('article');
  container.className = 'chapter-view';

  const isImage = chapter.chunk_type === 'IMG_' || chapter.chunk_type === 'COVR' || chapter.chunk_type === 'COVT';

  if (isImage) {
    const blob = new Blob([data], { type: 'image/jpeg' });
    const url = URL.createObjectURL(blob);
    const img = document.createElement('img');
    img.src = url;
    img.alt = chapter.alt_text || '';
    container.appendChild(img);
  } else if (chapter.chunk_type === 'SIDX' || chapter.chunk_type === 'MATH' || chapter.chunk_type === 'CSS_' || chapter.chunk_type === 'FONT') {
    container.innerHTML = '<p class="meta">[' + chapter.chunk_type + ' chunk ' + chapter.chunk_id + ' - ' + data.length + ' bytes]</p>';
  } else {
    const raw = new TextDecoder().decode(data);
    const isHtml = chapter.markup_type === 1;

    if (isHtml) {
      container.innerHTML = sanitizeHtml(raw);
    } else {
      container.innerHTML = renderHmd(raw);
    }
  }

  elements.viewer.appendChild(container);
  currentPage.set(currentChapterIndex + 1);
}

function sanitizeHtml(html) {
  const allowedTags = ['p','div','span','h1','h2','h3','h4','h5','h6','ul','ol','li','dl','dt','dd',
    'table','thead','tbody','tr','th','td','blockquote','pre','code','br','hr','em','strong','i','b',
    'u','s','sub','sup','img','a','figure','figcaption','section','article','header','main','aside'];
  const allowedAttrs = ['href','src','alt','title','class','id','name','target','rel','width','height'];
  const doc = new DOMParser().parseFromString(html, 'text/html');
  const walk = (node) => {
    if (node.nodeType === 3) return node;
    if (node.nodeType !== 1) { node.remove(); return null; }
    const tag = node.tagName.toLowerCase();
    if (!allowedTags.includes(tag)) {
      const span = document.createElement('span');
      while (node.firstChild) span.appendChild(node.firstChild);
      node.replaceWith(span);
      return span;
    }
    const keep = {};
    for (const attr of node.attributes) {
      if (allowedAttrs.includes(attr.name)) keep[attr.name] = attr.value;
    }
    while (node.attributes.length > 0) node.removeAttribute(node.attributes[0].name);
    for (const [k,v] of Object.entries(keep)) node.setAttribute(k, v);
    if (tag === 'a' && node.getAttribute('href')?.startsWith('http')) {
      node.setAttribute('target', '_blank');
      node.setAttribute('rel', 'noopener');
    }
    Array.from(node.childNodes).forEach(c => walk(c));
    return node;
  };
  Array.from(doc.body.childNodes).forEach(c => walk(c));
  return doc.body.innerHTML;
}

function renderHmd(text) {
  const lines = text.split('\n');
  const out = [];
  let inList = false;
  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed) { out.push(''); continue; }
    if (/^#{1,6}\s/.test(trimmed)) {
      const level = trimmed.match(/^(#+)/)[1].length;
      out.push('<' + 'h' + level + '>' + esc(trimmed.replace(/^#+\s*/, '')) + '</' + 'h' + level + '>');
    } else if (/^-\s/.test(trimmed)) {
      if (!inList) { out.push('<ul>'); inList = true; }
      out.push('<li>' + esc(trimmed.replace(/^-\s*/, '')) + '</li>');
    } else {
      if (inList) { out.push('</ul>'); inList = false; }
      out.push('<p>' + esc(trimmed) + '</p>');
    }
  }
  if (inList) out.push('</ul>');
  return out.join('\n');
}

function esc(s) {
  return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');
}

function generateToc() {
  if (!chapters.length || !elements.tocContent) return;
  elements.tocContent.innerHTML = '';
  chapters.forEach((ch, i) => {
    const item = document.createElement('div');
    item.className = 'toc-item';
    const label = ch.chunk_type + ' #' + ch.chunk_id + ' (' + ch.size_raw + ' bytes)';
    item.textContent = label;
    item.addEventListener('click', () => {
      currentChapterIndex = i;
      renderCurrentChapter();
      closeToc();
    });
    elements.tocContent.appendChild(item);
  });
}

function hasBookLoaded() { return chapters.length > 0; }

function clampChapterIndex(index) {
  if (!chapters.length) return 0;
  return Math.max(0, Math.min(index, chapters.length - 1));
}

function goToChapter(index) {
  if (!hasBookLoaded()) return;
  currentChapterIndex = clampChapterIndex(index);
  renderCurrentChapter();
}

function normalizedPageNumber() {
  const p = parseInt(elements.currentPageInput?.value ?? '', 10);
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
  if (e.key === 'ArrowLeft') prevPage();
  if (e.key === 'ArrowRight') nextPage();
}

export function toggleToc() {
  setTocOpen();
}

export function closeToc() {
  setTocOpen(false);
}
