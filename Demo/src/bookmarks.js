import { getBookmarks, removeBookmark } from "./state.js";
import { goToChapter } from "./book.js";
import { toggleSidebar } from "./state.js";

export function renderBookmarks(container, bookId, chapters) {
  const marks = getBookmarks(bookId);
  container.innerHTML = "";

  if (!marks.length) {
    container.innerHTML = `<div class="bookmarks-empty">
      <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" style="opacity:0.3"><path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"/></svg>
      <p>No bookmarks yet</p>
      <p class="bookmarks-hint">Use <kbd>⌘B</kbd> to bookmark the current chapter</p>
    </div>`;
    return;
  }

  marks.forEach((mark, i) => {
    const ch = chapters[mark.chapter];
    const label = ch
      ? `${ch.chunk_type} ${ch.chunk_id}`
      : `Chapter ${mark.chapter + 1}`;
    const note = mark.note || "";
    const date = new Date(mark.createdAt).toLocaleDateString(undefined, {
      month: "short",
      day: "numeric",
    });

    const item = document.createElement("div");
    item.className = "bookmark-item";
    item.innerHTML = `
      <div class="bookmark-info">
        <div class="bookmark-chapter">${esc(label)}</div>
        ${note ? `<div class="bookmark-note">${esc(note)}</div>` : ""}
        <div class="bookmark-date">${date}</div>
      </div>
      <button class="icon-btn bookmark-remove" data-index="${i}" aria-label="Remove bookmark">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
      </button>
    `;

    item.addEventListener("click", (e) => {
      if (e.target.closest(".bookmark-remove")) return;
      goToChapter(mark.chapter);
      toggleSidebar(null);
    });

    item.querySelector(".bookmark-remove").addEventListener("click", () => {
      removeBookmark(bookId, i);
      renderBookmarks(container, bookId, chapters);
    });

    container.appendChild(item);
  });
}

function esc(s) {
  return String(s)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}
