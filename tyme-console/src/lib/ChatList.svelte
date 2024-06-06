<script>
  import { onMount, onDestroy, afterUpdate } from "svelte";
  import { getChatMsg } from "../js/fetch";
  import { socket } from "../js/store.js";
  import { fade, fly } from "svelte/transition";
  import { backOut } from "svelte/easing";
  import { format, set } from "date-fns";

  /**
   * @typedef {Object} Topic
   * @property {string} id - The topic id.
   * @property {string} topic - The topic string.
   * @property {number} qos - The QoS value.
   */

  /**
   * @type {Topic}
   */
  export let header = {
    id: "",
    topic: "",
    qos: 0,
  };

  let msgs = [];
  let loading = true;

  $: if (header.topic !== undefined && header.topic !== "") {
    loading = true;
    msgs = [];
    getChatMsg(header.id).then((res) => {
     pushMsgs(res.data);
    });
  }

  const socketMessageListener = (/** @type {{ data: any; }} */ event) => {
    const data = JSON.parse(event.data);
    if (data.header.topic !== header.topic || data.header.qos !== header.qos) {
      return;
    }
    pushMsgs([data.msg]);
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

  /**
   * @param {any[]} msg
   */
  const pushMsgs = (msg) => {
    msgs = [...msg, ...msgs];
    loading = false;
  };
</script>

{#key header}
  <!--
    加了out会出现滚动条错误
    out:fade|local={{
      duration: 150,
      easing: backOut,
    }}  -->

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
        delay: 200,
        easing: backOut,
      }}
      out:fade|local={{
        duration: 150,
        easing: backOut,
      }}
    >
      {#each msgs as msg}
        <div class="p-2 w-full rounded-2xl">
          <p class="mb-2 text-sm text-primary-container dark:text-tertiary">
            <span class="">
              {format(Date.parse(msg.timestamp), "yyyy-MM-dd HH:mm:ss")}
            </span>
            <span class="ml-2">
              sender: <span class="font-semibold">{msg.sender}</span>
            </span>
          </p>
          {@html msg.content.html}
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
