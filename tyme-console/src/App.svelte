<script>
  import { user } from "./js/store.js";
  import NavBar from "./lib/Navbar.svelte";
  import Login from "./pages/Login.svelte";
  import { onMount } from "svelte";
  import Chat from "./pages/Chat.svelte";
  import Settings from "./pages/Settings.svelte";
  import Toasts from "./lib/Toasts.svelte";
  import Tasks from "./pages/Tasks.svelte";
  import ro from "date-fns/locale/ro";

  let routerId = 98;

  $: loggedin = $user !== "";

  const menuItems = (/** @type {boolean} */ loggedin) => {
    if (loggedin) {
      return [
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
    routerId =
      menuItems($user !== "").find((item) => item.label === currentHash)?.id ||
      98;
  }

  const handleLoginSuccess = () => {
    let currentHash = window.location.hash.substring(1);
    let router = menuItems($user !== "").find(
      (item) => item.label === currentHash,
    );
    router = router ? router : { label: "Chat", id: 4 };
    window.location.hash = router.label;
    routerId = router.id;
  };

  onMount(() => {
    let html = document.documentElement;
    html.setAttribute("data-theme", "light");
  });
</script>

<Toasts />

<!-- MENNU BAR ON TOP -->
<NavBar navItems={menuItems(loggedin)} bind:routerId />

<!-- PAGE LOADING -->
<div
  class="min-h-screen pt-16 md:pt-20 bg-gradient-to-r from-cyan-100 to-blue-100"
>
  <div class="w-full flex justify-center font-sans">
    {#if routerId === 0}
      <div class="w-1/2 flex justify-center" />
    {:else if routerId === 4}
      <Chat />
    {:else if routerId === 5}
      <Tasks />
    {:else if routerId === 6}
      <Settings />
    {:else if routerId === 98}
      <Login on:loginSuccess={handleLoginSuccess} />
    {:else}
      <h2>Page Not Found or Completed Yet</h2>
    {/if}
  </div>
</div>

<style>
</style>
