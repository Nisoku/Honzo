import {
  bindClass,
  bindDisabled,
  bindEvent,
  bindProperty,
  bindText,
} from "@nisoku/sairin";
import {
  bookTitle,
  canGoNext,
  canGoPrevious,
  currentPageText,
  errorClass,
  errorText,
  libraryClass,
  loadingClass,
  overlayClass,
  prevDisabled,
  nextDisabled,
  searchClass,
  searchDisabled,
  tocDisabled,
  tocClass,
  totalPagesText,
} from "./state.js";
import {
  closeToc,
  focusSearch,
  openBook,
  openBuiltinBook,
  goToPage,
  prevPage,
  nextPage,
  runSearch,
  setBookElements,
  toggleToc,
} from "./book.js";
import {
  handleLibraryFiles,
  openLibrary,
  setLibraryElements,
} from "./library.js";
import { hideError, toggleLibrary, toggleSearch } from "./ui.js";

const openButton = document.getElementById("open-button");
const openDemoButton = document.getElementById("open-demo-button");
const demoBookSelect = document.getElementById("demo-book-select");
const fileInput = document.getElementById("file-input");
const libraryInput = document.getElementById("library-input");
const libraryButton = document.getElementById("library-button");
const searchButton = document.getElementById("search-button");
const closeLibraryButton = document.getElementById("close-library");
const tocButton = document.getElementById("toc-button");
const closeTocButton = document.getElementById("close-toc");
const closeSearchButton = document.getElementById("close-search");
const prevButton = document.getElementById("prev-button");
const nextButton = document.getElementById("next-button");
const currentPageInput = document.getElementById("current-page");
const searchInput = document.getElementById("search-input");
const searchResults = document.getElementById("search-results");
const searchStatus = document.getElementById("search-status");
const overlay = document.getElementById("overlay");
const closeErrorButton = document.getElementById("close-error");
const bookTitleSpan = document.getElementById("book-title");
const totalPagesSpan = document.getElementById("total-pages");
const tocContainer = document.getElementById("toc-container");
const tocContent = document.getElementById("toc-content");
const searchContainer = document.getElementById("search-container");
const libraryContainer = document.getElementById("library-container");
const libraryContent = document.getElementById("library-content");
const loadingMessage = document.getElementById("loading-message");
const errorMessage = document.getElementById("error-message");
const errorTextNode = document.getElementById("error-text");

setBookElements({
  currentPageInput,
  searchInput,
  searchResults,
  searchStatus,
  tocContent,
  viewer: document.getElementById("viewer"),
});

setLibraryElements({
  libraryContent,
  libraryInput,
});

bindEvent(openButton, "click", () => fileInput.click());
bindEvent(openDemoButton, "click", () => {
  const selected = demoBookSelect.value;
  if (!selected) {
    return;
  }
  openBuiltinBook(selected);
});
bindEvent(fileInput, "change", openBook);
bindEvent(prevButton, "click", prevPage);
bindEvent(nextButton, "click", nextPage);
bindEvent(currentPageInput, "change", goToPage);
bindEvent(tocButton, "click", () => toggleToc());
bindEvent(closeTocButton, "click", () => closeToc());
bindEvent(searchButton, "click", () => {
  const opening = !searchContainer.classList.contains("open");
  toggleSearch();
  if (opening) {
    requestAnimationFrame(() => focusSearch());
  }
});
bindEvent(closeSearchButton, "click", () => toggleSearch(false));
bindEvent(searchInput, "input", (event) => runSearch(event.target.value));
bindEvent(libraryButton, "click", openLibrary);
bindEvent(closeLibraryButton, "click", () => toggleLibrary(false));
bindEvent(overlay, "click", () => {
  closeToc();
  toggleSearch(false);
  toggleLibrary(false);
  hideError();
});
bindEvent(closeErrorButton, "click", hideError);
bindEvent(libraryInput, "change", handleLibraryFiles);

bindText(bookTitleSpan, bookTitle);
bindText(totalPagesSpan, totalPagesText);
bindText(errorTextNode, errorText);

bindProperty(currentPageInput, "value", currentPageText);

bindDisabled(prevButton, prevDisabled);
bindDisabled(nextButton, nextDisabled);
bindDisabled(tocButton, tocDisabled);
bindDisabled(searchButton, searchDisabled);

bindClass(tocContainer, tocClass);
bindClass(searchContainer, searchClass);
bindClass(libraryContainer, libraryClass);
bindClass(overlay, overlayClass);
bindClass(loadingMessage, loadingClass);
bindClass(errorMessage, errorClass);
