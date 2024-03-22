<script>
  import { user } from "./js/store.js";
  import NavBar from "./lib/Navbar.svelte";
  import LogIn from "./pages/Login.svelte";
  import LogOut from "./pages/Logout.svelte";
  import Secure from "./pages/Secure.svelte";
  import ApiCheck from "./pages/ApiCheck.svelte";
  import { onMount } from "svelte";
  import About from "./pages/About.svelte";
  import Chat from "./pages/Chat.svelte";
  import Settings from "./pages/Settings.svelte";
  import Toasts from "./lib/Toasts.svelte";
  import Tasks from "./pages/Tasks.svelte";

  let menu = 98;

  $: loggedin = $user !== "";

  const set_menu_items = (/** @type {boolean} */ loggedin) => {
    if (loggedin) {
      return [
        { label: "About", id: 1 },
        { label: "Secure", id: 2 },
        { label: "CheckApi", id: 3 },
        { label: "Chat", id: 4 },
        { label: "Task", id: 5 },
        { label: "Setting", id: 6 },
        {
          label: "Logout",
          id: 99,
          icon: "solar:logout-linear",
          color: "text-red-500",
        },
      ];
    } else {
      return [
        { label: "About", id: 1 },
        { label: "CheckApi", id: 3 },
        {
          label: "Login",
          id: 98,
          icon: "solar:login-linear",
          color: "text-primary-500",
        },
      ];
    }
  };

  let currentHash = window.location.hash.substring(1);

  if (currentHash !== "") {
    menu =
      set_menu_items($user !== "").find((item) => item.label === currentHash)
        ?.id || 98;
  }

  onMount(() => {
    let html = document.documentElement;
    html.setAttribute("data-theme", "light");
  });
</script>

<Toasts />

<div class="h-screen flex flex-col bg-slate-500">
  <!-- MENNU BAR ON TOP -->
  <NavBar navItems={set_menu_items(loggedin)} bind:menu />

  <!-- PAGE LOADING -->
  <div
    class="overflow-auto flex-1 h-[calc(100vh-4rem)] md:h-[calc(100vh-5rem)]"
  >
    <div class="w-full flex justify-center font-sans">
      {#if menu === 0}
        <div class="w-1/2 flex justify-center" />
      {:else if menu === 1}
        <About />
      {:else if menu === 2}
        <Secure />
      {:else if menu === 3}
        <ApiCheck />
      {:else if menu === 4}
        <Chat />
      {:else if menu === 5}
        <Tasks />
      {:else if menu === 6}
        <Settings />
      {:else if menu === 98}
        <LogIn />
      {:else if menu === 99}
        <LogOut />
      {:else}
        <h2>Page Not Found or Completed Yet</h2>
      {/if}
    </div>
  </div>
</div>

<style>
</style>
