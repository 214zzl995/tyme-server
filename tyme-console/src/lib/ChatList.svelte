<script>
  import { onMount, onDestroy } from "svelte";
  import ChatCard from "./ChatCard.svelte";
  import { getChatMsg } from "../js/fetch";
  import { socket } from "../js/store.js";

  let pageNumber = 0;

  $: msgs = [];

  const socketMessageListener = (/** @type {{ data: any; }} */ event) => {
    const data = JSON.parse(event.data);
    msgs.push(data);
    console.log(msgs);
  };

  onMount(() => {
    getChatMsg().then((res) => {

      msgs = res.data;
      console.log(msgs);
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
</script>

<div class="w-full h-full overflow-y-scroll">
  {#each msgs as msg}
    <ChatCard {msg} />
  {/each}
</div>

<style>
</style>
