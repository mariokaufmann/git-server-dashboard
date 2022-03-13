import App from "./App.svelte";
import "modern-normalize/modern-normalize.css";
import "@fortawesome/fontawesome-free/css/solid.min.css";
import "@fortawesome/fontawesome-free/css/fontawesome.min.css";

const app = new App({
  target: document.getElementById("app"),
});

fetch("/api/version")
  .then((response) => response.text())
  .then(console.log);

export default app;
