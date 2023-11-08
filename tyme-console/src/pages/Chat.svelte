<script>
  import { onMount } from "svelte";
  import { getAllTopic } from "./../js/fetch.js";
  import Topic from "../lib/Topic.svelte";
  import Editor from "../lib/Editor.svelte";
  import ChatList from "../lib/ChatList.svelte";

  let topicList = [];
  let topicIndex = 1;

  onMount(() => {
    getAllTopic().then((res) => {
      topicList = res.topics;
    });
  });

  const changeTopic = (/** @type {{ detail: number; }} */ event) => {
    topicIndex = event.detail;
  };
</script>

<!-- 无法使用 grid 布局 尝试使用flex布局实现 -->
<div
  class=" w-11/12 sm:w-11/12 md:w-4/5 lg:w-3/4 h-chat flex flex-col md:flex-row"
>
  <div
    class="bg-white rounded shadow-md h-14 mb-3 md:mr-3 md:mb-0 flex-none md:h-full md:w-40"
  >
    <div class="flex flex-row md:flex-col p-2 h-full">
      {#each topicList as topic, index}
        <Topic text={topic} {index} {topicIndex} on:changeTopic={changeTopic} />
      {/each}
    </div>
  </div>

  <div class="bg-white rounded shadow-md flex-auto flex flex-col overflow-hidden">
    <div class="overflow-hidden">
      <span />
      <ChatList />
    </div>
    <div class="flex-none min-h-56">
      <Editor />
    </div>
  </div>
</div>
