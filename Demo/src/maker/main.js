import { bindEvent } from "@nisoku/sairin";
import { createIcons } from "lucide";
import { icons } from "../icons.js";
import { chunks, setChunks, layoutMode, setLayoutMode, defaultCompression, setDefaultCompression, direction, setDirection, chunksList, pmapBody, pmapEntries, addPmapBtn, buildBtn, layoutOptions, compressionOptions, directionOptions, showStatus, setDragSrcIdx, dragSrcIdx, setPmapEntries } from "./state.js";
import { createChunk, addChunk, removeChunk, moveChunk, renderChunks } from "./chunks.js";
import { addPmapEntry, removePmapEntry, renderPmap } from "./pmap.js";
import { addIdentifierRow, addContributorRow, addTag } from "./meta.js";
import { buildBook } from "./build.js";
import { deserializeState, saveProject, restoreProject, deleteSave, restoreAutosave, deleteAutosave, renderSaves, markDirty } from "./saves.js";

// Init
function setupFeaturePanels() {
  for (const [flagId, panelId] of [["flagDrm", "drmPanel"], ["flagAnno", "annoPanel"], ["flagSync", "syncPanel"]]) {
    const cb = document.getElementById(flagId);
    const panel = document.getElementById(panelId);
    if (cb && panel) {
      cb.addEventListener("change", () => panel.classList.toggle("active", cb.checked));
    }
  }
}

function initMaker() {
  createIcons({ icons });

  let restored = false;
  try {
    const raw = localStorage.getItem("honzo_maker_autosave");
    if (raw) {
      deserializeState(JSON.parse(raw));
      showStatus("success", "Restored auto-saved project");
      restored = true;
    }
  } catch (e) {}

  if (!restored) {
    setChunks([createChunk("CHAP", "Chapter 1")]);
    renderChunks();
    renderPmap();
  }

  setupFeaturePanels();
  renderSaves();

  const makerBody = document.querySelector(".maker-body");
  if (makerBody) {
    makerBody.addEventListener("input", markDirty);
    makerBody.addEventListener("change", markDirty);
  }
}

// ========================
// EVENT WIRING
// ========================

// Add chunk buttons
for (const [type, btnId] of [
  ["CHAP", "addChapBtn"], ["IMG_", "addImageBtn"], ["CSS_", "addCssBtn"],
  ["FONT", "addFontBtn"], ["COVR", "addCoverBtn"], ["MATH", "addMathBtn"],
  ["NOTE", "addNoteBtn"],
]) {
  const el = document.getElementById(btnId);
  if (el) bindEvent(el, "click", () => addChunk(type));
}

// Add PMAP
bindEvent(addPmapBtn, "click", addPmapEntry);

// Build
bindEvent(buildBtn, "click", buildBook);

// Save
const saveBtn = document.getElementById("saveBtn");
const saveNameInput = document.getElementById("saveNameInput");
if (saveBtn) {
  bindEvent(saveBtn, "click", () => {
    const name = saveNameInput?.value?.trim();
    if (!name) { showStatus("error", "Enter a save name"); return; }
    saveProject(name);
  });
}
if (saveNameInput) {
  bindEvent(saveNameInput, "keydown", (e) => {
    if (e.key === "Enter") saveBtn?.click();
  });
}

// Save action delegation
bindEvent(document, "click", (e) => {
  const btn = e.target.closest("[data-save-action]");
  if (!btn) return;
  const action = btn.dataset.saveAction;
  if (action === "restore-autosave") restoreAutosave();
  else if (action === "delete-autosave") deleteAutosave();
  else if (action === "restore" || action === "delete") {
    const name = btn.dataset.saveName;
    if (!name) return;
    if (action === "restore") restoreProject(name);
    else deleteSave(name);
  }
});

// Chunk event delegation
bindEvent(chunksList, "input", (e) => {
  const target = e.target;
  const card = target.closest(".chunk-card");
  if (!card) return;
  const id = parseInt(card.dataset.id, 10);
  const ch = chunks.find((c) => c.id === id);
  if (!ch) return;

  if (target.classList.contains("chunk-title-input")) ch.title = target.value;
  if (target.classList.contains("chunk-content")) ch.content = target.value;
  if (target.classList.contains("chunk-type-select")) {
    const textarea = card?.querySelector(".chunk-content");
    if (textarea) textarea.placeholder = `Write ${target.value} content here...`;
  }
  if (target.classList.contains("chunk-math-select")) {
    const textarea = card?.querySelector(".chunk-content");
    if (textarea) textarea.placeholder = parseInt(target.value, 10) ? "\\sum_{n=1}^{\\infty} ..." : "<math>...</math>";
  }
});

bindEvent(chunksList, "click", (e) => {
  const btn = e.target.closest("[data-action]");
  if (!btn) return;
  const action = btn.dataset.action;
  const id = parseInt(btn.dataset.id, 10);
  if (action === "delete") removeChunk(id);
  if (action === "settings") {
    const panel = document.getElementById(`settings_${id}`);
    if (panel) panel.style.display = panel.style.display === "none" ? "block" : "none";
  }
  if (action === "remove-file") {
    const ch = chunks.find((c) => c.id === id);
    if (ch) { ch.fileData = null; ch.fileName = ""; ch.fileType = ""; }
    renderChunks();
  }
});

// File input for binary chunks
bindEvent(chunksList, "change", (e) => {
  const input = e.target.closest(".chunk-file-input");
  if (!input) return;
  const id = parseInt(input.dataset.id, 10);
  const ch = chunks.find((c) => c.id === id);
  if (!ch || !input.files?.[0]) return;
  const file = input.files[0];
  const reader = new FileReader();
  reader.onload = () => {
    ch.fileData = new Uint8Array(reader.result);
    ch.fileName = file.name;
    ch.fileType = file.type || "";
    renderChunks();
  };
  reader.readAsArrayBuffer(file);
});

// File drop zone for binary chunks
bindEvent(chunksList, "dragover", (e) => {
  const drop = e.target.closest(".chunk-file-drop");
  if (drop) { e.preventDefault(); drop.classList.add("dragover"); }
});
bindEvent(chunksList, "dragleave", (e) => {
  const drop = e.target.closest(".chunk-file-drop");
  if (drop) drop.classList.remove("dragover");
});
bindEvent(chunksList, "drop", (e) => {
  const drop = e.target.closest(".chunk-file-drop");
  if (!drop) return;
  e.preventDefault();
  drop.classList.remove("dragover");
  const id = parseInt(drop.dataset.id, 10);
  const ch = chunks.find((c) => c.id === id);
  if (!ch || !e.dataTransfer?.files?.[0]) return;
  const file = e.dataTransfer.files[0];
  const reader = new FileReader();
  reader.onload = () => {
    ch.fileData = new Uint8Array(reader.result);
    ch.fileName = file.name;
    ch.fileType = file.type || "";
    renderChunks();
  };
  reader.readAsArrayBuffer(file);
});

// Drag reorder
bindEvent(chunksList, "dragstart", (e) => {
  const card = e.target.closest(".chunk-card");
  if (!card) return;
  setDragSrcIdx(parseInt(card.dataset.idx, 10));
  e.dataTransfer.effectAllowed = "move";
  e.dataTransfer.setData("text/plain", String(dragSrcIdx));
  card.classList.add("dragging");
});
bindEvent(chunksList, "dragend", (e) => {
  const card = e.target.closest(".chunk-card");
  if (card) card.classList.remove("dragging");
  document.querySelectorAll(".chunk-card.drag-over").forEach((el) => el.classList.remove("drag-over"));
  setDragSrcIdx(null);
});
bindEvent(chunksList, "dragover", (e) => {
  if (e.target.closest(".chunk-file-drop")) return;
  e.preventDefault();
  e.dataTransfer.dropEffect = "move";
  const target = e.target.closest(".chunk-card");
  if (!target || dragSrcIdx === null) return;
  document.querySelectorAll(".chunk-card.drag-over").forEach((el) => el.classList.remove("drag-over"));
  target.classList.add("drag-over");
});
bindEvent(chunksList, "drop", (e) => {
  if (e.target.closest(".chunk-file-drop")) return;
  e.preventDefault();
  const target = e.target.closest(".chunk-card");
  if (!target || dragSrcIdx === null) return;
  target.classList.remove("drag-over");
  moveChunk(dragSrcIdx, parseInt(target.dataset.idx, 10));
  setDragSrcIdx(null);
});

// Settings selectors
const setupSegmented = (el, cb) => {
  if (!el) return;
  bindEvent(el, "click", (e) => {
    const opt = e.target.closest(".setting-option");
    if (!opt) return;
    el.querySelectorAll(".setting-option").forEach((o) => o.classList.remove("active"));
    opt.classList.add("active");
    cb(parseInt(opt.dataset.value, 10));
  });
};
setupSegmented(layoutOptions, (v) => setLayoutMode(v));
setupSegmented(compressionOptions, (v) => setDefaultCompression(v));

if (directionOptions) {
  bindEvent(directionOptions, "click", (e) => {
    const opt = e.target.closest(".setting-option");
    if (!opt) return;
    directionOptions.querySelectorAll(".setting-option").forEach((o) => o.classList.remove("active"));
    opt.classList.add("active");
    setDirection(opt.dataset.value);
  });
}

// PMAP events
bindEvent(pmapBody, "input", (e) => {
  const target = e.target;
  if (!target.classList.contains("pmap-input")) return;
  const idx = parseInt(target.dataset.idx, 10);
  const val = parseInt(target.value, 10) || 0;
  const entries = [...pmapEntries];
  if (target.classList.contains("pmap-print")) entries[idx].printPage = val || 1;
  else if (target.classList.contains("pmap-chunk")) entries[idx].chunkId = val;
  else if (target.classList.contains("pmap-offset")) entries[idx].byteOffset = val;
  setPmapEntries(entries);
});
bindEvent(pmapBody, "click", (e) => {
  const btn = e.target.closest("[data-action='delete-pmap']");
  if (btn) removePmapEntry(parseInt(btn.dataset.idx, 10));
});

// Identifiers
bindEvent(document.getElementById("addIdBtn"), "click", () => addIdentifierRow());

// Contributors
bindEvent(document.getElementById("addContribBtn"), "click", () => addContributorRow());

// Tags
bindEvent(document, "click", (e) => {
  const btn = e.target.closest("[data-tag-add]");
  if (btn) addTag(btn.dataset.tagAdd);
  const rem = e.target.closest(".tag-remove");
  if (rem) rem.parentElement.remove();
});

// Delegate remove for identifier/contributor rows
bindEvent(document, "click", (e) => {
  const btn = e.target.closest("[data-remove]");
  if (btn) btn.closest(".maker-id-row, .maker-contrib-row")?.remove();
});

// Prevent file drop on body from reloading page
bindEvent(document.body, "dragover", (e) => { if (e.dataTransfer?.types.includes("Files")) e.preventDefault(); });
bindEvent(document.body, "drop", (e) => { if (e.dataTransfer?.types.includes("Files")) e.preventDefault(); });

// Start
initMaker();
