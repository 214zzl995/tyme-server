<script>
  import "iconify-icon";
  import { getLogout } from "./../js/auth.js";
  import appIco from "../assets/icons/app_ico.svg";
  import { onMount } from "svelte";
  import { getAllTopic } from "../js/fetch.js";
  import { topicActive, changeActiveTopic } from "../js/store.js";
  import { getStorageValue } from "../js/storage.js";
  import { fade } from "svelte/transition";
  import Modal from "./Modal.svelte";

  export let topicDialogShow;

  const Logout = async () => {
    await getLogout();
    localStorage.removeItem("user");
    window.location.href = "/";
  };

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
  let loading = true;

  let addTopicModalShow = false;

  onMount(() => {
    getAllTopic().then((res) => {
      getStorageValue("topic-active")
        .then((storage) => {
          if (storage) {
            changeActiveTopic(storage);
          } else if (res?.topics?.length > 0) {
            changeActiveTopic(res.topics[0]);
          }
        })
        .finally(() => {
          topicList = res.topics;
          loading = false;
        });
    });
  });
</script>

<nav class="px-2 sm:px-4 py-2.5 z-20 h-full backdrop-blur-xl flex flex-col">
  <div class="flex flex-row items-center">
    <img src={appIco} alt="appIco" class="h-16 w-16" />
    <p class="leading-[4rem] text-2xl font-bold mb-2.5">Tyme</p>
  </div>

  <div class="mt-8">
    <button
      class="interactive-bg-secondary-container p-4 rounded-lg flex flex-row items-center gap-2 hover:shadow-md"
      on:click={() => {
        topicDialogShow = !topicDialogShow;
      }}
    >
      <iconify-icon icon="line-md:plus-circle" width="1.5em" height="1.5em"
      ></iconify-icon>
      Add Topic
    </button>
  </div>
  {#if loading}
    <div
      out:fade={{
        duration: 150,
      }}
      class="w-full flex justify-center mt-20"
    >
      <iconify-icon
        icon="svg-spinners:pulse-3"
        width="3em"
        height="3em"
        class="text-primary"
      ></iconify-icon>
    </div>
  {:else}
    <ul
      class="flex gap-2 justify-start flex-col flex-1 mt-5"
      in:fade={{
        delay: 200,
      }}
    >
      {#each topicList as topic}
        <li>
          <div>
            <button
              class="bg-transparent p-2.5 pl-4 rounded-full flex flex-row items-center gap-1 hover:bg-secondary-container/50 w-full"
              class:topic-active={topic.id === $topicActive?.id}
              on:click={() => {
                if (topicDialogShow) {
                  topicDialogShow = false;
                }
                changeActiveTopic(topic);
              }}
            >
              <iconify-icon
                icon="line-md:bell"
                width="1em"
                height="1em"
                class="mr-2"
              ></iconify-icon>
              {topic.topic}
            </button>
          </div>
        </li>
      {/each}
    </ul>
  {/if}
</nav>

<Modal bind:open={addTopicModalShow} size="sm" title={"Add Topic"}>
  <div>
    <div class="gap-4 grid grid-cols-[40px__1fr]">
      <label for="topic" class="font-semibold self-center">Topic</label>
      <input
        type="text"
        id="topic"
        class="p-2.5 rounded-lg w-full topic-input"
      />
      <label for="qos" class="font-semibold self-center">QoS</label>
      <select id="qos" class="p-2.5 rounded-lg w-full topic-input">
        <option value="0">0</option>
        <option value="1">1</option>
        <option value="2">2</option>
      </select>
    </div>

    <div class="flex flex-row-reverse gap-2 mt-4">
      <button
        class="interactive-bg-secondary p-2.5 rounded-lg"
        on:click={() => {
          addTopicModalShow = false;
        }}
      >
        Cancel
      </button>
      <button class="interactive-bg-primary p-2.5 rounded-lg"> Add </button>
    </div>
  </div>
</Modal>

<style lang="postcss">
  .topic-active {
    @apply bg-secondary-container/50;
  }
  .topic-input {
    @apply bg-secondary-container/50 text-on-secondary-container border-0 outline-none rounded-xl p-4 transition-all;
  }
</style>
