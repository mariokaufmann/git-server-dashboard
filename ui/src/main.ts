import App from "./App.svelte";
import "modern-normalize/modern-normalize.css";
import "@fortawesome/fontawesome-free/css/solid.min.css";
import "@fortawesome/fontawesome-free/css/fontawesome.min.css";

import relativeTime from "dayjs/plugin/relativeTime";
import dayjs from "dayjs";

dayjs.extend(relativeTime);

const app = new App({
  target: document.getElementById("app"),
});

fetch("/api/version")
  .then((response) => response.text())
  .then((serverVersion) => console.log(`Server version: ${serverVersion}.`));

export default app;
