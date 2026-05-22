import { derived, path, signal } from "@nisoku/sairin";

export const loadingVisible = signal(path("ui", "loadingVisible"), false);
export const errorVisible = signal(path("ui", "errorVisible"), false);
export const errorText = signal(
  path("ui", "errorText"),
  "There was an error processing your file.",
);
export const libraryOpen = signal(path("ui", "libraryOpen"), false);
export const tocOpen = signal(path("ui", "tocOpen"), false);
export const searchOpen = signal(path("ui", "searchOpen"), false);

export const bookTitle = signal(path("reader", "bookTitle"), "");
export const currentPage = signal(path("reader", "currentPage"), 1);
export const totalPages = signal(path("reader", "totalPages"), 1);
export const hasBook = signal(path("reader", "hasBook"), false);

export const canGoPrevious = derived(path("reader", "canGoPrevious"), () => {
  return hasBook.get() && currentPage.get() > 1;
});

export const canGoNext = derived(path("reader", "canGoNext"), () => {
  return hasBook.get() && currentPage.get() < totalPages.get();
});

export const prevDisabled = derived(path("reader", "prevDisabled"), () => {
  return !canGoPrevious.get();
});

export const nextDisabled = derived(path("reader", "nextDisabled"), () => {
  return !canGoNext.get();
});

export const tocDisabled = derived(path("reader", "tocDisabled"), () => {
  return !hasBook.get();
});

export const searchDisabled = derived(path("reader", "searchDisabled"), () => {
  return !hasBook.get();
});

export const currentPageText = derived(path("reader", "currentPageText"), () => {
  return String(currentPage.get());
});

export const totalPagesText = derived(path("reader", "totalPagesText"), () => {
  return String(totalPages.get());
});

export const loadingClass = derived(path("ui", "loadingClass"), () => {
  return `message${loadingVisible.get() ? " show" : ""}`;
});

export const errorClass = derived(path("ui", "errorClass"), () => {
  return `message${errorVisible.get() ? " show" : ""}`;
});

export const overlayClass = derived(path("ui", "overlayClass"), () => {
  const open =
    libraryOpen.get() || tocOpen.get() || searchOpen.get() || loadingVisible.get() || errorVisible.get();
  return `overlay${open ? " open" : ""}`;
});

export const libraryClass = derived(path("ui", "libraryClass"), () => {
  return `library-container${libraryOpen.get() ? " open" : ""}`;
});

export const tocClass = derived(path("ui", "tocClass"), () => {
  return `toc-container${tocOpen.get() ? " open" : ""}`;
});

export const searchClass = derived(path("ui", "searchClass"), () => {
  return `search-container${searchOpen.get() ? " open" : ""}`;
});

export function resetReaderState() {
  bookTitle.set("");
  currentPage.set(1);
  totalPages.set(1);
  hasBook.set(false);
  searchOpen.set(false);
}
