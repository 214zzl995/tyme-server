<script>
  import "iconify-icon";
  import { getLogout } from "./../js/auth.js";
  import appIco from "../assets/icons/app_ico.svg";
  import { onMount } from "svelte";
  import { getAllTopic } from "../js/fetch.js";
  import { topicActive, changeActiveTopic } from "../js/store.js";
  import { getStorageValue } from "../js/storage.js";
  import { fade } from "svelte/transition";

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

<style lang="postcss">
  .topic-active {
    @apply bg-secondary-container/50;
  }
</style>
