<script>
  import { user } from "./js/store.js";
  import { getSession } from "./js/auth.js";
  import NavBar from "./lib/Navbar.svelte";
  import LogIn from "./pages/Login.svelte";
  import LogOut from "./pages/Logout.svelte";
  import Secure from "./pages/Secure.svelte";
  import ApiCheck from "./pages/ApiCheck.svelte";
  import { onMount } from "svelte";
  import About from "./pages/About.svelte";
  import Chat from "./pages/Chat.svelte";
  import Settings from "./pages/Settings.svelte";

  let menu = 2;

  $: loggedin = $user !== "";

  const set_menu_items = (/** @type {boolean} */ loggedin) => {
    if (loggedin) {
      return [
        { label: "About", id: 1 },
        { label: "Secure", id: 3 },
        { label: "Logout", id: 4 },
        { label: "CheckApi", id: 5 },
        { label: "Chat", id: 6 },
        { label: "Setting", id: 7 },
      ];
    } else {
      return [
        { label: "About", id: 1 },
        { label: "Login", id: 2 },
        { label: "CheckApi", id: 5 },
      ];
    }
  };

  let currentHash = window.location.hash.substring(1);

  if (currentHash !== "") {
    menu = set_menu_items(loggedin).find(
      (item) => item.label === currentHash
    )?.id;
  }

  // check if logged in
  onMount(() => {
    getSession();
    let html = document.documentElement;
    html.setAttribute("data-theme", "light");
  });
</script>

<div class="h-screen flex flex-col">
  <!-- MENNU BAR ON TOP -->
  <NavBar navItems={set_menu_items(loggedin)} bind:menu />

  <!-- PAGE LOADING -->
  <div class="overflow-auto bg-green-50 pt-5 flex-1">
    <div class="w-full flex justify-center">
      {#if menu === 1}
        <About />
      {:else if menu === 2}
        <LogIn />
      {:else if menu === 3}
        <Secure />
      {:else if menu === 4}
        <LogOut />
      {:else if menu === 5}
        <ApiCheck />
      {:else if menu === 6}
        <Chat />
      {:else if menu === 7}
        <Settings />
      {:else}
        <h2>Page Not Found or Completed Yet</h2>
      {/if}
    </div>
  </div>
</div>

<style>
</style>
