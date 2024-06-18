<script>
  import { user, guide } from "./js/store.js";
  import NavBar from "./lib/Navbar.svelte";
  import Login from "./pages/Login.svelte";
  import Chat from "./pages/Chat.svelte";
  import Toasts from "./lib/Toasts.svelte";
  import Guide from "./pages/Guide.svelte";
  import { fly } from "svelte/transition";
  import { cubicInOut } from "svelte/easing";

  $: loggedin = $user !== "";

  let topicDialogShow = false;
</script>

<Toasts />

{#if $guide}
  <Guide />
{:else}
  <div class="h-[-webkit-fill-available] w-full font-sans">
    {#if !loggedin}
      <Login />
    {:else}
      <div
        class="grid grid-cols-[250px_minmax(0,_1fr)] grid-rows-1 gap-4 bg-surface"
      >
        <div class="h-screen col-span-1">
          <NavBar bind:topicDialogShow />
        </div>
        <div class="flex-1 col-span-1 relative overflow-hidden">
          <!-- 出现方向性错误了 需要把Chat拆分为 Header 和 Main Main中需要添加AddTopic和 Task页面 未来还有 ignitor 触发器界面-->
          {#if topicDialogShow}
            <div
              class="absolute top-0 right-0 bg-surface h-full w-full z-[998] rounded-l-2xl"
              transition:fly={{
                x: 500,
                duration: 300,
                easing: cubicInOut,
              }}
            ></div>
          {/if}
          <Chat />
        </div>
      </div>
    {/if}
  </div>
{/if}
