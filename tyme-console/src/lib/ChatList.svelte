<script>
  import { onMount, onDestroy } from "svelte";
  import { getPageMessagesByHeader } from "../js/fetch";
  import { socket } from "../js/store.js";
  import { fade, fly } from "svelte/transition";
  import { backOut } from "svelte/easing";
  import ChatItem from "./ChatItem.svelte";

  /**
   * @type {import("../js/store.js").Header}
   */
  export let header;
  export let currentPageNum;
  export let pageSize;

  let msgs = [];
  let loading = true;

  $: if (currentPageNum !== undefined && currentPageNum !== -1) {
    getMessages();
  }

  $: if (header !== undefined) {
    loading = true;
  }

  const getMessages = () => {
    getPageMessagesByHeader(header.id, {
      page_num: currentPageNum,
      page_size: pageSize,
    })
      .then((res) => {
        msgs = res.data;
      })
      .finally(() => {
        if (loading) {
          loading = false;
        }
      });
  };

  const socketMessageListener = (/** @type {{ data: any; }} */ event) => {
    const data = JSON.parse(event.data);
    if (data.header.topic !== header.topic || data.header.qos !== header.qos) {
      return;
    }
  };

  onMount(() => {
    if ($socket) {
      $socket.getWebSocket.addEventListener(
        "message",
        socketMessageListener,
        false
      );
    }
  });

  onDestroy(() => {
    if ($socket) {
      $socket.getWebSocket.removeEventListener(
        "message",
        socketMessageListener,
        false
      );
    }
  });
</script>

{#key header}
  {#if loading}
    <div class="w-full h-full flex justify-center items-center">
      <iconify-icon
        icon="svg-spinners:pulse-3"
        width="3em"
        height="3em"
        class="text-primary"
      ></iconify-icon>
    </div>
  {:else}
    <div
      class="w-full msgs-container p-5"
      in:fly={{
        y: 50,
        delay: 100,
        easing: backOut,
      }}
      out:fade={{
        duration: 100,
        easing: backOut,
      }}
    >
      {#each msgs as msg}
        <div>
          <ChatItem {msg} />
        </div>
      {/each}
    </div>
  {/if}
{/key}

<style lang="postcss">
  .msgs-container div:nth-child(odd) {
    @apply bg-surface-bright;
  }
</style>
