import { openBookFromEntry } from "./book.js";
import { showError } from "./ui.js";
import init, { HonzoWasm } from "../wasm/honzo_wasm.js";

let elLibraryContent = null;
let elLibraryInput = null;
let wasmInitPromise = null;

async function ensureWasmReady() {
  if (!wasmInitPromise) wasmInitPromise = init();
  await wasmInitPromise;
}

export function setLibraryElements(elements) {
  elLibraryContent = elements.libraryContent || elLibraryContent;
  elLibraryInput = elements.libraryInput || elLibraryInput;
}

export async function openLibrary() {
  try {
    const files = [];
    if ("showDirectoryPicker" in window) {
      const dirHandle = await window.showDirectoryPicker();
      for await (const entry of dirHandle.values()) {
        if (entry.kind === "file" && entry.name.endsWith(".hzo")) {
          files.push(entry);
        }
      }
    } else {
      elLibraryInput?.click();
      return;
    }
    displayLibraryGrid(files);
  } catch (err) {
    showError(
      "Failed to open library: " +
        (err instanceof Error ? err.message : String(err)),
    );
  }
}

export function handleLibraryFiles(e) {
  displayLibraryGrid(Array.from(e.target.files));
}

async function displayLibraryGrid(fileEntries) {
  if (!elLibraryContent) return;
  elLibraryContent.innerHTML = "";
  if (fileEntries.length === 0) {
    elLibraryContent.textContent = "No .hzo files found.";
    return;
  }
  for (const entry of fileEntries) {
    const item = await createLibraryItem(entry);
    elLibraryContent.appendChild(item);
  }
}

async function createLibraryItem(fileEntry) {
  const item = document.createElement("div");
  item.className = "toc-item";
  item.style.marginBottom = "2px";

  const label = document.createElement("span");
  label.className = "toc-label";
  label.textContent = fileEntry.name;
  item.appendChild(label);

  try {
    const file =
      typeof fileEntry.getFile === "function"
        ? await fileEntry.getFile()
        : fileEntry;
    if (file.name.endsWith(".hzo")) {
      const buf = await file.arrayBuffer();
      await ensureWasmReady();
      const reader = new HonzoWasm(new Uint8Array(buf), 1);
      const meta = reader.get_meta_parsed();
      const firstVal = (v) =>
        v instanceof Map
          ? v.values().next().value
          : typeof v === "object"
            ? Object.values(v)[0]
            : v;
      label.textContent = firstVal(meta?.title) || fileEntry.name;
    }
  } catch (err) {
    console.error("Error reading metadata for", fileEntry.name, err);
  }

  item.addEventListener("click", () => openBookFromEntry(fileEntry));
  return item;
}
