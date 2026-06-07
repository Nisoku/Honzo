export function showLoading() {
  const el = document.getElementById("loading-message");
  if (el) el.classList.add("show");
}

export function hideLoading() {
  const el = document.getElementById("loading-message");
  if (el) el.classList.remove("show");
}

export function showError(message) {
  const el = document.getElementById("error-message");
  const text = document.getElementById("error-text");
  if (text) text.textContent = message;
  if (el) el.classList.add("show");
}

export function hideError() {
  const el = document.getElementById("error-message");
  if (el) el.classList.remove("show");
}
