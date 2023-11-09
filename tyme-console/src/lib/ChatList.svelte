<script>
  import { onMount, onDestroy } from "svelte";
  import ChatCard from "./ChatCard.svelte";
  import { getChatMsg } from "../js/fetch";
  import { socket } from "../js/store.js";

  let pageNumber = 0;

  let divRef;

  $: msgs = [];

  const socketMessageListener = (/** @type {{ data: any; }} */ event) => {
    const data = JSON.parse(event.data);
    pushMsgs([data]);
  };

  const scrollToBottom = () => {
    if (divRef) {
          //滚动到对应位置 
    }
  };

  onMount(() => {
    getChatMsg().then((res) => {
      pushMsgs(res.data);
    });

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
  function pushMsgs(msg) {
    msgs = [...msgs, ...msg]; // 使用展开运算符创建一个新数组，以便 Svelte 能够检测到变化
    scrollToBottom();
  }
</script>

<div class="w-full h-full overflow-y-scroll" bind:this={divRef}>
  {#each msgs as msg}
    <ChatCard {msg} />
  {/each}
</div>

<style>
</style>
