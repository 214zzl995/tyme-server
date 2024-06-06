<script>
  import { topicActive } from "../js/store.js";
  import { fade, fly } from "svelte/transition";
  import ChatList from "../lib/ChatList.svelte";
  import {  cubicIn, cubicOut } from "svelte/easing";
</script>

<div class="w-full h-screen flex flex-col gap-8">
  <div class="h-16 w-full bg-on-inverse-surface rounded-l-2xl mt-5 relative">
    <div class="flex flex-col justify-center w-min absolute left-5 top-2">
      {#key $topicActive?.topic}
        <p
          class="text-xl font-black"
          in:fade|local={{
            delay: 100,
            easing: cubicIn,
          }}
          out:fade|local={{
            duration: 150,
            easing: cubicOut,
          }}
        >
          {$topicActive === undefined ? "" : $topicActive.topic}
        </p>
        <p
          class="text-sm font-black text-outline"
          in:fade|local={{
            delay: 100,
            easing: cubicIn,
          }}
          out:fade|local={{
            duration: 150,
            easing: cubicOut,
          }}
        >
          {$topicActive === undefined ? "" : "qos:" + $topicActive?.qos}
        </p>
      {/key}
    </div>
  </div>

  <div
    class="flex-1 w-full bg-white dark:bg-on-inverse-surface rounded-tl-2xl overflow-auto"
  >
    <ChatList header={$topicActive} />
  </div>
</div>

<style>
</style>
