<script>
  import { onMount, onDestroy } from "svelte";
  import ChatCard from "./ChatCard.svelte";
  import { getChatMsg } from "../js/fetch";
  import { socket } from "../js/store.js";

  export let header = "";

  let pageNumber = 0;

  let divRef;

  $: msgs = [];

  $: {
    if (header !== "") {
      msgs = [];
      getChatMsg(header).then((res) => {
        pushMsgs(res.data);
        scrollToBottom(false);
      });
    }
  }

  const socketMessageListener = (/** @type {{ data: any; }} */ event) => {
    const data = JSON.parse(event.data);
    if(data.topic.header !== header){
      return;
    }
    pushMsgs([data]);
    scrollToBottom(true);
  };

  const scrollToBottom = (/** @type {Boolean?} */ isAnimation) => {
    if (divRef) {
      requestAnimationFrame(() =>
        divRef.scrollBy({
          top: divRef.scrollHeight,
          behavior: isAnimation ? "smooth" : "auto" ,
        })
      );
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

  /**
   * @param {any[]} msg
   */
  const pushMsgs = (msg) => {
    msgs = [...msgs, ...msg]; // 使用展开运算符创建一个新数组，以便 Svelte 能够检测到变化
  };
</script>

<div class="w-full h-full overflow-y-scroll" bind:this={divRef}>
  {#each msgs as msg}
    <ChatCard {msg} />
  {/each}
</div>

<style>
</style>
