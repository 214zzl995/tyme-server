<script>
  import { onMount } from "svelte";
  import { getAllTopic } from "./../js/fetch.js";
  import Topic from "../lib/Topic.svelte";
  import Editor from "../lib/Editor.svelte";
  import ChatList from "../lib/ChatList.svelte";

  /**
   * @typedef {Object} Topic
   * @property {string} id - The topic id.
   * @property {string} topic - The topic string.
   * @property {number} qos - The QoS value.
   */

  /**
   * @type {Array<Topic>}
   */
  let topicList = [];
  let topicIndex = -1;

  $: {
    if (topicIndex != -1) {
      sessionStorage.setItem("topicPage", topicList[topicIndex].topic);
    }
  }

  onMount(() => {
    getAllTopic().then((res) => {
      topicList = res.topics;

      const url = new URL(window.location.href);
      const topic = sessionStorage.getItem("topicPage");
      if (topic != "null") {
        topicIndex = topicList.findIndex((item) => item.topic == topic);

        if (topicIndex == -1 && topicList.length > 0) {
          topicIndex = 0;
        }
      } else {
        topicIndex = 0;
      }
    });
  });
</script>

<!-- 无法使用 grid 布局 尝试使用flex布局实现 -->
<div
  class="w-full sm:w-full md:w-4/5 lg:w-3/4 h-[calc(100vh-4rem)] md:h-[calc(100vh-7rem)] flex flex-col md:flex-row md:mt-3"
>
  <div
    class="bg-white md:rounded md:shadow-md h-10 md:h-14 mb:mb-3 md:mr-3 md:mb-0 flex-none md:h-full md:w-40 border-b md:border-b-0 border-gray-200"
  >
    <div class="flex flex-row md:flex-col px-2 py-0.5 h-full topics">
      {#each topicList as topic, index}
        <Topic
          text={topic.topic}
          {index}
          {topicIndex}
          on:changeTopic={() => (topicIndex = index)}
        />
      {/each}
    </div>
  </div>

  <div
    class="bg-white md:rounded md:shadow-md flex-auto flex flex-col justify-between overflow-hidden"
  >
    <div class="overflow-hidden">
      <span />
      {#if topicIndex !== -1}
        <ChatList header={topicList[topicIndex]} />
      {/if}
    </div>
    <div class="flex-none h-48 md:h-64">
      <Editor header={topicList[topicIndex]} />
    </div>
  </div>
</div>

<style>
  .topics {
    overflow: overlay;
  }
</style>
