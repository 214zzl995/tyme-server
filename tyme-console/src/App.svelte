<script>
  import { user, guide } from "./js/store.js";
  import NavBar from "./lib/Navbar.svelte";
  import Login from "./pages/Login.svelte";
  import Chat from "./pages/Chat.svelte";
  import Toasts from "./lib/Toasts.svelte";
  import Guide from "./pages/Guide.svelte";

  $: loggedin = $user !== "";

  const menuItems = [
    { label: "Chat", id: 4 },
    { label: "Task", id: 5 },
  ];
</script>

<Toasts />

{#if $guide}
  <Guide />
{:else}
  <div class="h-[-webkit-fill-available] w-full font-sans">
    {#if !loggedin}
      <Login />
    {:else}
      <div class="grid grid-cols-[250px_minmax(0,_1fr)] grid-rows-1 gap-4 bg-surface">
        <div class="h-screen col-span-1">
          <NavBar />
        </div>
        <div class="flex-1 col-span-1 ">
           <Chat />
        </div>
      </div>
    {/if}
  </div>
{/if}
