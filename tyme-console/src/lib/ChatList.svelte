<script>
  import { onMount } from "svelte";
  import ChatCard from "./ChatCard.svelte";
  import { getChatMsg } from "../js/fetch";

  let pageNumber = 0;

  $: sources = [];

  onMount(() => {
    getChatMsg().then((res) => {
      // sources.push(res.data);
      sources = res.data;
      console.log(sources);
    });
  });
</script>

<div class="w-full h-full overflow-y-scroll">
  {#each sources as source}
    <ChatCard source={source.content.html} mine={source.mine} />
  {/each}
</div>

<style>
</style>
