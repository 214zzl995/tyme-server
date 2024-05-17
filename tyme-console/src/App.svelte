<script>
  import { user, guide } from "./js/store.js";
  import NavBar from "./lib/Navbar.svelte";
  import Login from "./pages/Login.svelte";
  import { onMount } from "svelte";
  import Chat from "./pages/Chat.svelte";
  import Toasts from "./lib/Toasts.svelte";
  import Tasks from "./pages/Tasks.svelte";
  import Guide from "./pages/Guide.svelte";

  let routerId = 98;

  $: loggedin = $user !== "";

  const menuItems = [
    { label: "Chat", id: 4 },
    { label: "Task", id: 5 },
  ];

  let currentHash = window.location.hash.substring(1);

  if ($user !== "") {
    routerId =
      currentHash !== ""
        ? menuItems.find((item) => item.label === currentHash)?.id || 4
        : 4;
  } else {
    routerId = 98;
  }

  const handleLoginSuccess = () => {
    console.log("Login Success");
    let currentHash = window.location.hash.substring(1);
    let router;
    if (currentHash !== "") {
      router = menuItems.find((item) => item.label === currentHash);
    } else {
      router = { label: "Chat", id: 4 };
    }

    window.location.hash = router.label;
    routerId = router.id;
  };

  onMount(() => {
    let html = document.documentElement;
    html.setAttribute("data-theme", "light");
  });
</script>

<Toasts />

{#if $guide}
  <Guide />
{:else}
  <div class="h-[-webkit-fill-available] w-full font-sans">
    {#if !loggedin}
      <Login on:loginSuccess={handleLoginSuccess} />
    {:else}
      <div class = "grid grid-cols-12 grid-rows-1">
        <div class="h-screen z-50 col-span-1">
          <NavBar navItems={menuItems} bind:routerId />
        </div>
        <div class="flex-1 col-span-11">
          {#if routerId === 4}
            <Chat />
          {:else if routerId === 5}
            <Tasks />
          {:else}
            <h2>Page Not Found or Completed Yet</h2>
          {/if}
        </div>
      </div>
    {/if}
  </div>
{/if}
