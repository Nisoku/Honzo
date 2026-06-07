import { icon } from "./icons.js";
import { getBookmarks, removeBookmark } from "./state.js";
import { goToChapter } from "./book.js";
import { toggleSidebar } from "./state.js";

export function renderBookmarks(container, bookId, chapters) {
  const marks = getBookmarks(bookId);
  container.innerHTML = "";

  if (!marks.length) {
    container.innerHTML = `<div class="bookmarks-empty">
      <div style="opacity:0.3">${icon("Bookmark", 32, 1.5)}</div>
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
        ${icon("X", 14)}
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
