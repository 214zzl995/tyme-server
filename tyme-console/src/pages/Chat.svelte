<script>
  import { onDestroy, onMount } from "svelte";
  import { getAllTopic } from "./../js/fetch.js";
  import Topic from "../lib/Topic.svelte";
  import Editor from "../lib/Editor.svelte";
  import ChatList from "../lib/ChatList.svelte";

  /** @type {string[]} */
  let topicList = [];
  let topicIndex = -1;

  onMount(() => {
    getAllTopic().then((res) => {
      topicList = res.topics;

      const url = new URL(window.location.href);
      const topic = decodeURIComponent(url.searchParams.get("topic"));
      if (topic != "null") {
        topicIndex = topicList.indexOf(topic);

        if (topicIndex == -1) {
          topicIndex = 0;
        }
      } else {
        topicIndex = 0;
      }
      changeTopic({ detail: topicIndex });
    });
  });

  const changeTopic = (/** @type {{ detail: number; }} */ event) => {
    topicIndex = event.detail;
    const loc = window.location;
    let url = `${loc.origin}${loc.pathname}?topic=${encodeURIComponent(
      topicList[topicIndex]
    )}${loc.hash}`;

    var state = { page: "about" };
    var title = "about";
    window.history.replaceState(state, title, url);

  };
</script>

<!-- 无法使用 grid 布局 尝试使用flex布局实现 -->
<div
  class="w-full sm:w-full md:w-4/5 lg:w-3/4 h-[calc(100vh-4rem)] md:h-[calc(100vh-7rem)] flex flex-col md:flex-row md:mt-3"
>
  <div
    class="bg-white md:rounded md:shadow-md h-14 mb:mb-3 md:mr-3 md:mb-0 flex-none md:h-full md:w-40 border-b md:border-b-0 border-gray-200"
  >
    <div class="flex flex-row md:flex-col p-2 h-full">
      {#each topicList as topic, index}
        <Topic text={topic} {index} {topicIndex} on:changeTopic={changeTopic} />
      {/each}
    </div>
  </div>

  <div
    class="bg-white md:rounded md:shadow-md flex-auto flex flex-col justify-between overflow-hidden"
  >
    <div class="overflow-hidden">
      <span />
      <ChatList header={topicList[topicIndex]} />
    </div>
    <div class="flex-none h-48 md:h-64">
      <Editor header = {topicList[topicIndex]}/>
    </div>
  </div>
</div>
