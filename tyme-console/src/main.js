import "./app.postcss";
// @ts-ignore
import App from "./App.svelte";
import { getSession } from "./js/auth";

async function initializeApp() {
  await getSession();
  const app = new App({
    target: document.getElementById("app"),
  });
  return app;
}

export default initializeApp();
