import { derived, path, signal } from "@nisoku/sairin";

// UI State
export const loadingVisible = signal(path("ui", "loadingVisible"), false);
export const errorVisible = signal(path("ui", "errorVisible"), false);
export const errorText = signal(path("ui", "errorText"), "");
export const overlayVisible = signal(path("ui", "overlayVisible"), false);

// Open sidebar panels (only one at a time, using overlap)
export const activeSidebar = signal(path("ui", "activeSidebar"), null); // null | 'toc' | 'bookmarks' | 'search' | 'meta' | 'settings'

// Reader State
export const bookTitle = signal(path("reader", "bookTitle"), "");
export const currentChapter = signal(path("reader", "currentChapter"), 0);
export const totalChapters = signal(path("reader", "totalChapters"), 0);
export const hasBook = signal(path("reader", "hasBook"), false);
export const currentChunkId = signal(path("reader", "currentChunkId"), 0);
export const chapterLabel = signal(path("reader", "chapterLabel"), "");
export const currentDirection = signal(
  path("reader", "currentDirection"),
  "ltr",
);
export const noBook = derived(path("reader", "noBook"), () => !hasBook.get());

export const canGoPrev = derived(
  path("reader", "canGoPrev"),
  () => hasBook.get() && currentChapter.get() > 0,
);

export const canGoNext = derived(
  path("reader", "canGoNext"),
  () => hasBook.get() && currentChapter.get() < totalChapters.get() - 1,
);

export const prevDisabled = derived(
  path("reader", "prevDisabled"),
  () => !canGoPrev.get(),
);

export const nextDisabled = derived(
  path("reader", "nextDisabled"),
  () => !canGoNext.get(),
);

export const currentPageText = derived(path("reader", "currentPageText"), () =>
  String(currentChapter.get() + 1),
);

export const totalPagesText = derived(path("reader", "totalPagesText"), () =>
  String(totalChapters.get()),
);

export const progressPct = derived(path("reader", "progressPct"), () => {
  const t = totalChapters.get();
  if (t <= 1) return 0;
  return ((currentChapter.get() + 1) / t) * 100;
});

// Settings State
function loadSetting(key, defaultVal) {
  try {
    const v = localStorage.getItem("honzo_" + key);
    return v !== null ? JSON.parse(v) : defaultVal;
  } catch {
    return defaultVal;
  }
}
function saveSetting(key, val) {
  try {
    localStorage.setItem("honzo_" + key, JSON.stringify(val));
  } catch {}
}

export const theme = signal(
  path("settings", "theme"),
  loadSetting("theme", "light"),
);
theme.subscribe(() => saveSetting("theme", theme.get()));

export const fontSize = signal(
  path("settings", "fontSize"),
  loadSetting("fontSize", "medium"),
);
fontSize.subscribe(() => saveSetting("fontSize", fontSize.get()));

export const fontFamily = signal(
  path("settings", "fontFamily"),
  loadSetting("fontFamily", "sans"),
);
fontFamily.subscribe(() => saveSetting("fontFamily", fontFamily.get()));

export const layoutMode = signal(
  path("settings", "layoutMode"),
  loadSetting("layoutMode", "scroll"),
);
layoutMode.subscribe(() => saveSetting("layoutMode", layoutMode.get()));

export const textAlign = signal(
  path("settings", "textAlign"),
  loadSetting("textAlign", "ltr"),
);
textAlign.subscribe(() => saveSetting("textAlign", textAlign.get()));

// Reading Progress
function loadProgress(bookId) {
  try {
    const d = localStorage.getItem("honzo_progress_" + bookId);
    return d ? JSON.parse(d) : null;
  } catch {
    return null;
  }
}
function saveProgress(bookId, data) {
  try {
    localStorage.setItem("honzo_progress_" + bookId, JSON.stringify(data));
  } catch {}
}

export function getProgress(bookId) {
  return loadProgress(bookId);
}

export function setProgress(bookId, chapterIndex, chunkId) {
  saveProgress(bookId, {
    chapter: chapterIndex,
    chunkId,
    updatedAt: Date.now(),
  });
}

export function clearProgress(bookId) {
  try {
    localStorage.removeItem("honzo_progress_" + bookId);
  } catch {}
}

// Bookmarks
function loadBookmarks(bookId) {
  try {
    const d = localStorage.getItem("honzo_bookmarks_" + bookId);
    return d ? JSON.parse(d) : [];
  } catch {
    return [];
  }
}
function saveBookmarks(bookId, marks) {
  try {
    localStorage.setItem("honzo_bookmarks_" + bookId, JSON.stringify(marks));
  } catch {}
}

export function getBookmarks(bookId) {
  return loadBookmarks(bookId);
}

export function addBookmark(bookId, chapterIndex, chunkId, note) {
  const marks = loadBookmarks(bookId);
  marks.push({
    chapter: chapterIndex,
    chunkId,
    note: note || "",
    createdAt: Date.now(),
  });
  saveBookmarks(bookId, marks);
  return marks;
}

export function removeBookmark(bookId, index) {
  const marks = loadBookmarks(bookId);
  if (index >= 0 && index < marks.length) {
    marks.splice(index, 1);
    saveBookmarks(bookId, marks);
  }
  return marks;
}

export function updateBookmarkNote(bookId, index, note) {
  const marks = loadBookmarks(bookId);
  if (index >= 0 && index < marks.length) {
    marks[index].note = note;
    saveBookmarks(bookId, marks);
  }
  return marks;
}

// Sidebar helper
export function toggleSidebar(name) {
  const current = activeSidebar.get();

  activeSidebar.set(current === name ? null : name);
}

export function closeSidebar() {
  activeSidebar.set(null);
}

// Book ID (derived from file name + size, used for persistence)
let currentBookId = null;
export function setCurrentBookId(id) {
  currentBookId = id;
}
export function getCurrentBookId() {
  return currentBookId;
}

export function resetReaderState() {
  bookTitle.set("");
  currentChapter.set(0);
  totalChapters.set(0);
  hasBook.set(false);
  currentChunkId.set(0);
  chapterLabel.set("");
  activeSidebar.set(null);
}
