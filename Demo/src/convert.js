import initHonzo, { convert_epub } from "./wasm/honzo_wasm.js";
import { bindClass, bindEvent, bindStyle, bindText } from "@nisoku/sairin";
import { derived, path, signal } from "@nisoku/sairin";

let honzoReady = false;

const dropZone = document.getElementById("dropZone");
const fileInput = document.getElementById("fileInput");
const statusEl = document.getElementById("status");
const statusText = document.getElementById("statusText");
const progressFill = document.getElementById("progressFill");
const historyList = document.getElementById("historyList");

const statusVisible = signal(path("convert", "statusVisible"), false);
const statusKind = signal(path("convert", "statusKind"), "");
const statusMessage = signal(path("convert", "statusMessage"), "");
const progressWidth = signal(path("convert", "progressWidth"), "0%");

const statusClass = derived(path("convert", "statusClass"), () => {
  return `status${statusVisible.get() ? ` active ${statusKind.get()}` : ""}`;
});

bindClass(statusEl, statusClass);
bindText(statusText, statusMessage);
bindStyle(progressFill, "width", progressWidth);

bindEvent(dropZone, "click", () => fileInput.click());
bindEvent(dropZone, "dragover", (event) => {
  event.preventDefault();
  dropZone.classList.add("dragover");
});
bindEvent(dropZone, "dragleave", () => dropZone.classList.remove("dragover"));
bindEvent(dropZone, "drop", (event) => {
  event.preventDefault();
  dropZone.classList.remove("dragover");
  handleFile(event.dataTransfer?.files?.[0]);
});
bindEvent(fileInput, "change", (event) => {
  handleFile(event.target.files?.[0]);
});

async function ensureHonzo() {
  if (!honzoReady) {
    await initHonzo();
    honzoReady = true;
  }
}

async function handleFile(file) {
  if (!file) {
    return;
  }

  showStatus("loading", `Reading ${file.name}...`, 10);

  try {
    const buf = await file.arrayBuffer();
    showStatus("loading", "Converting EPUB...", 40);

    await ensureHonzo();
    const hzo = convert_epub(new Uint8Array(buf));
    showStatus(
      "success",
      `Converted: ${hzo.length.toLocaleString()} bytes`,
      100,
    );
    addToHistory(file.name.replace(/\.epub$/i, ""), hzo);
  } catch (error) {
    showStatus("error", `Error: ${error?.message || error}`, 0);
  }
}

function showStatus(kind, message, progress) {
  statusVisible.set(true);
  statusKind.set(kind);
  statusMessage.set(message);
  progressWidth.set(`${progress}%`);
}

function addToHistory(name, data) {
  const item = document.createElement("div");
  item.className = "history-item";

  const filename = name.replace(/[^a-zA-Z0-9_-]/g, "_") + ".hzo";
  const blob = new Blob([data], { type: "application/octet-stream" });
  const url = URL.createObjectURL(blob);

  item.innerHTML = `
    <div>
      <div class="name">${filename}</div>
      <div class="size">${data.length.toLocaleString()} bytes</div>
    </div>
    <a class="dl-btn" href="${url}" download="${filename}">Download</a>
  `;

  historyList.prepend(item);
}
