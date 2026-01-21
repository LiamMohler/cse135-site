document.addEventListener("DOMContentLoaded", () => {
  const timestamp = document.getElementById("timestamp");
  if (timestamp) {
    timestamp.textContent = new Date().toString();
  }
});
