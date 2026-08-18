import {
  esc,
  formatSize,
  CHUNK_TYPES,
  chunks,
  chunkIdCounter,
  setChunkIdCounter,
  setChunks,
  chunksList,
  defaultCompression,
  showStatus,
} from "./state.js";
import { markDirty } from "./saves.js";
import { icon } from "../icons.js";

export function createChunk(type, title) {
  const id = chunkIdCounter + 1;
  setChunkIdCounter(id);
  const base = {
    id,
    type,
    title: title || type,
    compression: defaultCompression,
  };
  if (CHUNK_TYPES[type]?.markup) {
    base.content = "";
    base.contentType = "markdown";
    base.altText = null;
  } else if (type === "IMG_" || type === "COVR") {
    base.fileData = null;
    base.fileName = "";
    base.fileType = "";
    base.altText = null;
    base.coverType = type === "COVR" ? 0 : undefined;
  } else if (type === "CSS_") {
    base.content = "";
  } else if (type === "FONT") {
    base.fileData = null;
    base.fileName = "";
    base.fileType = "";
    base.fontEmbedding = 0;
    base.fontLicenseUrl = "";
  } else if (type === "MATH") {
    base.content = "";
    base.mathType = 0;
  }
  return base;
}

export function addChunk(type) {
  const label = CHUNK_TYPES[type]?.label || type;
  const idx = chunks.length;
  const arr = [
    ...chunks,
    createChunk(type, `${label} ${idx === 0 ? 1 : idx + 1}`),
  ];
  setChunks(arr);
  renderChunks();
}

export function removeChunk(id) {
  if (chunks.length <= 1) {
    showStatus("error", "Must have at least one chunk");
    return;
  }
  setChunks(chunks.filter((c) => c.id !== id));
  renderChunks();
}

export function moveChunk(fromIdx, toIdx) {
  if (toIdx < 0 || toIdx >= chunks.length) return;
  const arr = [...chunks];
  const [item] = arr.splice(fromIdx, 1);
  arr.splice(toIdx, 0, item);
  setChunks(arr);
  renderChunks();
}

export function updateChunkField(id, field, value) {
  const ch = chunks.find((c) => c.id === id);
  if (ch) ch[field] = value;
}

export function renderChunks() {
  chunksList.innerHTML = chunks
    .map((ch, i) => {
      const t = CHUNK_TYPES[ch.type] || { label: ch.type };
      return `<div class="chunk-card" draggable="true" data-idx="${i}" data-id="${ch.id}">
        <div class="chunk-header">
          <div class="chunk-drag" title="Drag to reorder">${icon("GripVertical", 16)}</div>
          <span class="chunk-type-badge" data-type="${ch.type}">${ch.type}</span>
          <span class="chunk-title-display">${esc(ch.title)}</span>
          <div class="chunk-header-right">
            <span class="chunk-comp-badge">${ch.compression ? "LZ4" : "Raw"}</span>
            <button class="chunk-settings-btn icon-btn" data-action="settings" data-id="${ch.id}" title="Chunk settings">${icon("Settings", 14)}</button>
            <button class="chunk-delete-btn icon-btn" data-action="delete" data-id="${ch.id}" title="Remove chunk">${icon("Trash2", 14)}</button>
          </div>
        </div>
        ${renderChunkBody(ch, i)}
        <div class="chunk-settings-panel" id="settings_${ch.id}" style="display:none">
          <div class="chunk-settings-inner">
            <label>Compression: <select class="chunk-sel-comp" data-id="${ch.id}">
              <option value="0" ${ch.compression === 0 ? "selected" : ""}>None</option>
              <option value="1" ${ch.compression === 1 ? "selected" : ""}>LZ4</option>
            </select></label>
            ${
              ch.type === "COVR" || ch.type === "IMG_"
                ? `
            <label>Cover Type: <select class="chunk-sel-cover" data-id="${ch.id}">
              <option value="0" ${ch.coverType === 0 ? "selected" : ""}>Front</option>
              <option value="1" ${ch.coverType === 1 ? "selected" : ""}>Back</option>
              <option value="2" ${ch.coverType === 2 ? "selected" : ""}>Full Spread</option>
            </select></label>`
                : ""
            }
            ${
              ch.type === "IMG_" || ch.type === "COVR"
                ? `
            <label>Alt Text: <input type="text" class="chunk-input-alt" value="${esc(ch.altText || "")}" placeholder="Image description" data-id="${ch.id}" /></label>`
                : ""
            }
            ${
              ch.type === "FONT"
                ? `
            <label>Embedding: <select class="chunk-sel-embed" data-id="${ch.id}">
              <option value="0" ${ch.fontEmbedding === 0 ? "selected" : ""}>Allowed</option>
              <option value="1" ${ch.fontEmbedding === 1 ? "selected" : ""}>Print Only</option>
              <option value="2" ${ch.fontEmbedding === 2 ? "selected" : ""}>No Modify</option>
              <option value="3" ${ch.fontEmbedding === 3 ? "selected" : ""}>No Embed</option>
            </select></label>
            <label>License URL: <input type="text" class="chunk-input-license" value="${esc(ch.fontLicenseUrl || "")}" placeholder="https://..." data-id="${ch.id}" /></label>`
                : ""
            }
          </div>
        </div>
      </div>`;
    })
    .join("");
  markDirty();
}

function renderChunkBody(ch, i) {
  const t = CHUNK_TYPES[ch.type];
  if (!t) return "";
  if (t.markup) {
    return `<div class="chunk-body">
      <div class="chunk-title-row">
        <input type="text" class="chunk-title-input" value="${esc(ch.title)}" placeholder="Title" data-id="${ch.id}" />
        <select class="chunk-type-select" data-id="${ch.id}">
          <option value="markdown" ${ch.contentType === "markdown" ? "selected" : ""}>Markdown</option>
          <option value="html" ${ch.contentType === "html" ? "selected" : ""}>HTML</option>
        </select>
      </div>
      <textarea class="chunk-content" data-id="${ch.id}" placeholder="Write ${ch.contentType} content here..." rows="8">${esc(ch.content)}</textarea>
      <div class="chunk-toolbar">
        <button class="btn btn-xs chunk-pagebreak-btn" data-action="insert-pagebreak" data-id="${ch.id}" title="Insert page break marker">
          ${icon("FilePlus", 12)} Page Break
        </button>
      </div>
    </div>`;
  }
  if (t.binary) {
    const isCover = ch.type === "COVR";
    const isFont = ch.type === "FONT";
    const accept = isFont ? "font/*,.ttf,.otf,.woff,.woff2" : "image/*";
    return `<div class="chunk-body">
      <div class="chunk-title-row">
        <input type="text" class="chunk-title-input" value="${esc(ch.title)}" placeholder="${isCover ? "Cover title" : isFont ? "Font name" : "Image title"}" data-id="${ch.id}" />
      </div>
      <div class="chunk-file-area">
        ${
          ch.fileData
            ? `
        <div class="chunk-file-preview">
          <span class="chunk-file-name">${esc(ch.fileName)} (${formatSize(ch.fileData.length)})</span>
          <button class="btn btn-secondary" data-action="remove-file" data-id="${ch.id}" style="font-size:0.8rem;padding:4px 10px;height:auto">Remove</button>
        </div>`
            : `
        <div class="chunk-file-drop" data-id="${ch.id}">
          <span>Drop ${isCover ? "cover image" : isFont ? "font" : "image"} or click to browse</span>
          <input type="file" class="chunk-file-input" accept="${accept}" data-id="${ch.id}" />
        </div>`
        }
      </div>
    </div>`;
  }
  if (t.text) {
    return `<div class="chunk-body">
      <div class="chunk-title-row">
        <input type="text" class="chunk-title-input" value="${esc(ch.title)}" placeholder="Stylesheet name" data-id="${ch.id}" />
      </div>
      <textarea class="chunk-content chunk-css-content" data-id="${ch.id}" placeholder="/* CSS */" rows="8" spellcheck="false">${esc(ch.content)}</textarea>
    </div>`;
  }
  if (t.math) {
    return `<div class="chunk-body">
      <div class="chunk-title-row">
        <input type="text" class="chunk-title-input" value="${esc(ch.title)}" placeholder="Math title" data-id="${ch.id}" />
        <select class="chunk-math-select" data-id="${ch.id}">
          <option value="0" ${ch.mathType === 0 ? "selected" : ""}>MathML</option>
          <option value="1" ${ch.mathType === 1 ? "selected" : ""}>LaTeX</option>
        </select>
      </div>
      <textarea class="chunk-content chunk-math-content" data-id="${ch.id}" placeholder="${ch.mathType ? "\\sum_{n=1}^{\\infty} ..." : "<math>...</math>"}" rows="8" spellcheck="false">${esc(ch.content)}</textarea>
    </div>`;
  }
  return `<div class="chunk-body"><div class="chunk-title-row">
    <input type="text" class="chunk-title-input" value="${esc(ch.title)}" placeholder="Name" data-id="${ch.id}" />
  </div></div>`;
}
