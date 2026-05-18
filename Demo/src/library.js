import { openBookFromEntry } from "./book.js";
import { showError, toggleLibrary } from "./ui.js";
import init, { HonzoWasm } from "./wasm/honzo_wasm.js";

let elements = {
  libraryContent: null,
  libraryInput: null,
};
let wasmInitPromise = null;

async function ensureWasmReady() {
  if (!wasmInitPromise) {
    wasmInitPromise = init();
  }
  await wasmInitPromise;
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

export function setLibraryElements(nextElements) {
  elements = {
    ...elements,
    ...nextElements,
  };
}

export async function openLibrary() {
  try {
    const files = [];
    if ('showDirectoryPicker' in window) {
      const dirHandle = await window.showDirectoryPicker();
      for await (const entry of dirHandle.values()) {
        if (entry.kind === 'file' && entry.name.endsWith('.hzo')) {
          files.push(entry);
        }
      }
    } else {
      elements.libraryInput?.click();
      return;
    }
    displayLibraryGrid(files);
    toggleLibrary(true);
  } catch (err) {
    showError('Failed to open library: ' + formatError(err));
  }
}

export function handleLibraryFiles(e) {
  const files = Array.from(e.target.files);
  displayLibraryGrid(files);
  toggleLibrary(true);
}

async function displayLibraryGrid(fileEntries) {
  if (!elements.libraryContent) return;
  elements.libraryContent.innerHTML = '';
  if (fileEntries.length === 0) {
    elements.libraryContent.textContent = 'No .hzo files found.';
    return;
  }
  for (const entry of fileEntries) {
    const item = await createLibraryItem(entry);
    elements.libraryContent.appendChild(item);
  }
}

async function createLibraryItem(fileEntry) {
  const item = document.createElement('div');
  item.className = 'library-item';

  const titleDiv = document.createElement('div');
  titleDiv.className = 'library-title';
  titleDiv.textContent = fileEntry.name;
  item.appendChild(titleDiv);

  try {
    const file = typeof fileEntry.getFile === 'function' ? await fileEntry.getFile() : fileEntry;
    if (file.name.endsWith('.hzo')) {
      const buf = await file.arrayBuffer();
      await ensureWasmReady();
      const reader = new HonzoWasm(new Uint8Array(buf), 1);
      const meta = reader.get_meta_parsed();
      const firstVal = (v) => v instanceof Map ? v.values().next().value : (typeof v === 'object' ? Object.values(v)[0] : v);
      titleDiv.textContent = firstVal(meta?.title) || fileEntry.name;
    }
  } catch (err) {
    console.error('Error reading metadata for', fileEntry.name, err);
  }

  item.addEventListener('click', () => openBookFromEntry(fileEntry));
  return item;
}


