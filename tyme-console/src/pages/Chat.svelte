<script>
  import { topicActive } from "../js/store.js";
  import { fade } from "svelte/transition";
  import ChatList from "../lib/ChatList.svelte";
  import { cubicIn, cubicOut, quintOut } from "svelte/easing";
  import { getMessagesCountByHeader } from "../js/fetch.js";
  import { flip } from "svelte/animate";

  const PAGE_SIZE = 20;
  const DISPLAY_PAGE_NUM = 5;

  let maxPageNum;
  let currentPageNum;

  topicActive.subscribe(
    (
      /**
       * @type {import("../js/store.js").Header}
       */ value
    ) => {
      currentPageNum = -1;
      if (value === undefined) {
        return;
      }
      getMessagesCountByHeader(value.id).then((res) => {
        const count = res.count;
        const pages = Math.ceil(count / PAGE_SIZE);
        maxPageNum = pages;
        currentPageNum = 0;
      });
    }
  );

  $: pageArray = Array.from(
    { length: Math.min(DISPLAY_PAGE_NUM, maxPageNum) },
    (_, i) =>
      maxPageNum - currentPageNum < DISPLAY_PAGE_NUM / 2
        ? maxPageNum - Math.min(DISPLAY_PAGE_NUM, maxPageNum) + i
        : currentPageNum +
          i -
          Math.min(currentPageNum, Math.floor(DISPLAY_PAGE_NUM / 2))
  );
</script>

<div class="w-full h-screen flex flex-col gap-2">
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

  <div class="h-6 flex justify-end items-center">
    {#each pageArray as num (num)}
      <button
        class="p-1 w-11 transition duration-200 rounded-full text-sm"
        animate:flip={{ duration: 300, easing: quintOut }}
        class:page_num_active={num === currentPageNum}
        class:page_num_inactive={num !== currentPageNum}
        on:click={() => {
          currentPageNum = num;
        }}
      >
        {num + 1}
      </button>
    {/each}
    <button
      on:click={() => {
        currentPageNum = Math.max(0, currentPageNum - 1);
      }}
      class="flex items-center justify-center rounded-full p-2 hover:bg-secondary ml-3 text-base"
    >
      <iconify-icon icon="line-md:chevron-small-left"></iconify-icon>
    </button>

    <button
      class="flex items-center justify-center rounded-full p-2 hover:bg-secondary text-base mr-8"
      on:click={() => {
        currentPageNum = Math.min(maxPageNum - 1, currentPageNum + 1);
      }}
    >
      <iconify-icon icon="line-md:chevron-small-right"></iconify-icon>
    </button>
  </div>

  <div
    class="flex-1 w-full bg-white dark:bg-on-inverse-surface rounded-tl-2xl overflow-auto"
  >
    <ChatList header={$topicActive} pageSize={PAGE_SIZE} {currentPageNum} />
  </div>
</div>

<style lang="postcss">
  .page_num_active {
    @apply bg-secondary/50 text-on-secondary;
  }
  .page_num_inactive {
  }
</style>
