import { bindDisabled, bindEvent, bindText } from "@nisoku/sairin";
import { createIcons } from "lucide";
import { icons } from "./icons.js";
import {
  activeSidebar,
  bookTitle,
  chapterLabel,
  closeSidebar,
  currentPageText,
  errorText,
  hasBook,
  nextDisabled,
  prevDisabled,
  referencePage,
  textAlign,
  totalPagesText,
  toggleSidebar,
} from "./state.js";
import {
  focusSearch,
  nextPage,
  openBook,
  openBuiltinBook,
  prevPage,
  runSearch,
  setBookElements,
} from "./book.js";
import { hideError } from "./ui.js";
import { renderSettings, applyTheme } from "./settings.js";

applyTheme();

// DOM refs
const openButton = document.getElementById("open-button");
const fileInput = document.getElementById("file-input");
const tocButton = document.getElementById("toc-button");
const bookmarksButton = document.getElementById("bookmarks-button");
const searchButton = document.getElementById("search-button");
const metaButton = document.getElementById("meta-button");
const settingsButton = document.getElementById("settings-button");
const prevButton = document.getElementById("prev-button");
const nextButton = document.getElementById("next-button");
const searchInput = document.getElementById("search-input");
const searchResults = document.getElementById("search-results");
const searchStatus = document.getElementById("search-status");
const overlay = document.getElementById("overlay");
const closeErrorButton = document.getElementById("close-error");
const bookTitleSpan = document.getElementById("book-title");
const totalPagesSpan = document.getElementById("total-pages");
const currentPageSpan = document.getElementById("current-page");
const tocSidebar = document.getElementById("toc-sidebar");
const tocContent = document.getElementById("toc-content");
const bookmarksSidebar = document.getElementById("bookmarks-sidebar");
const bookmarksContent = document.getElementById("bookmarks-content");
const searchSidebar = document.getElementById("search-sidebar");
const metaSidebar = document.getElementById("meta-sidebar");
const metaContent = document.getElementById("meta-content");
const settingsSidebar = document.getElementById("settings-sidebar");
const settingsContent = document.getElementById("settings-content");
const errorTextNode = document.getElementById("error-text");
const chapterLabelSpan = document.getElementById("chapter-label");
const refPageSpan = document.getElementById("ref-page");
const progressFill = document.getElementById("progress-fill");
const footer = document.getElementById("app-footer");

// Demo select
const demoBookSelect = document.getElementById("demo-book-select");

setBookElements({
  searchInput,
  searchResults,
  searchStatus,
  tocContent,
  viewer: document.getElementById("viewer"),
  chapterLabel: chapterLabelSpan,
  progressFill,
  footer,
  bookmarksContent,
  metaContent,
});

// Sidebar open/close via class toggle
const sidebarEls = {
  toc: tocSidebar,
  bookmarks: bookmarksSidebar,
  search: searchSidebar,
  meta: metaSidebar,
  settings: settingsSidebar,
};

activeSidebar.subscribe(() => {
  const name = activeSidebar.get();
  Object.values(sidebarEls).forEach((el) => el.classList.remove("open"));
  if (name && sidebarEls[name]) sidebarEls[name].classList.add("open");
  overlay.classList.toggle("open", name !== null);
});

// Events
bindEvent(openButton, "click", () => fileInput.click());
bindEvent(fileInput, "change", openBook);
if (demoBookSelect) {
  bindEvent(demoBookSelect, "change", () => {
    const val = demoBookSelect.value;
    if (val) openBuiltinBook(val);
    demoBookSelect.value = "";
  });
}

bindEvent(prevButton, "click", prevPage);
bindEvent(nextButton, "click", nextPage);

// Sidebar toggles
bindEvent(tocButton, "click", () => toggleSidebar("toc"));
bindEvent(bookmarksButton, "click", () => toggleSidebar("bookmarks"));
bindEvent(searchButton, "click", () => {
  toggleSidebar("search");
  setTimeout(() => focusSearch(), 50);
});
bindEvent(metaButton, "click", () => toggleSidebar("meta"));
bindEvent(settingsButton, "click", () => {
  toggleSidebar("settings");
  renderSettings(settingsContent);
});

// Close buttons
const closeButtons = {
  toc: document.getElementById("close-toc"),
  bookmarks: document.getElementById("close-bookmarks"),
  search: document.getElementById("close-search"),
  meta: document.getElementById("close-meta"),
  settings: document.getElementById("close-settings"),
};
Object.values(closeButtons).forEach((btn) =>
  bindEvent(btn, "click", closeSidebar),
);

bindEvent(searchInput, "input", (e) => runSearch(e.target.value));
bindEvent(overlay, "click", closeSidebar);
bindEvent(closeErrorButton, "click", hideError);

document.addEventListener("keydown", (e) => {
  if (e.key === "Escape") closeSidebar();
});

// Text bindings
bindText(bookTitleSpan, bookTitle);
bindText(totalPagesSpan, totalPagesText);
bindText(currentPageSpan, currentPageText);
bindText(errorTextNode, errorText);
bindText(chapterLabelSpan, chapterLabel);
bindText(refPageSpan, referencePage);

// Disabled bindings for prev/next
bindDisabled(prevButton, prevDisabled);
bindDisabled(nextButton, nextDisabled);

// Sidebar buttons
function updateSidebar(v) {
  [
    tocButton,
    bookmarksButton,
    searchButton,
    metaButton,
    settingsButton,
  ].forEach((b) => {
    b.disabled = !v;
  });
}
updateSidebar(false);
hasBook.subscribe(() => updateSidebar(hasBook.get()));

// Text alignment on body
textAlign.subscribe(() => {
  document.body.setAttribute("data-text-align", textAlign.get());
});

// Lucide icons
createIcons({ icons });

// Initial render
renderSettings(settingsContent);
