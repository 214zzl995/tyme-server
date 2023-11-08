<script>
  import { onMount } from "svelte";
  import ChatCard from "./ChatCard.svelte";
  import { getChatMsg } from "../js/fetch";

  let pageNumber = 0;

  $: msgs = [];

  onMount(() => {
    getChatMsg().then((res) => {
      // sources.push(res.data);
      msgs = res.data;
      console.log(msgs);
    });
  });
</script>

<div class="w-full h-full overflow-y-scroll">
  {#each msgs as msg}
    <ChatCard {msg} />
  {/each}
</div>

<style>
</style>
