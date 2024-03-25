import "./app.postcss";
import App from "./App.svelte";
import Start from "./Start.svelte";
import { getSession } from "./js/auth";
import { getFirstStart } from "./js/fetch";

async function initializeApp() {
  //在初始化之前 getSession 初始化store中的user 不然很难改同步
  getFirstStart().then((res) => {
    if (res.first_start) {
      const start = new Start({
        target: document.getElementById("app"),
      });
      return start;
    } else {
      getSession().then((res) => {
        const app = new App({
          target: document.getElementById("app"),
        });
        return app;
      });
    }
  });


}

export default initializeApp();
