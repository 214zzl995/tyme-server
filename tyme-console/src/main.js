import "./app.postcss";
import App from "./App.svelte";
import { getSession } from "./js/auth";

import Guide from "./pages/Guide.svelte";

async function initializeApp() {
  getSession().then(() => {
    const app = new App({
      target: document.getElementById("app"),
    });
    return app;
  });
}

export default initializeApp();
