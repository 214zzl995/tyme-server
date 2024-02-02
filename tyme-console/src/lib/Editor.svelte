<script>
  import Button from "flowbite-svelte/Button.svelte";
  import "iconify-icon";
  import EditorSwitch from "./EditorSwitch.svelte";
  import { addToast } from "./../js/store.js";
  import { sendMsg } from "./../js/fetch.js";
  import Spinner from "flowbite-svelte/Spinner.svelte";
  import { onMount } from "svelte";

  /**
   * @typedef {Object} Topic
   * @property {string} topic - The topic string.
   * @property {number} qos - The QoS value.
   */

  /**
   * @type {Topic}
   */
  export let header = {
    topic: "",
    qos: 0,
  };

  let text = "";
  let type;
  let topicError = false;
  let inputTopic = "";
  let submitSussess = false;
  let submitLoading = false;

  const pattern = /^[a-zA-Z]+\/#$/;

  $: {
    if (inputTopic !== "") {
      localStorage.setItem("inputTopic", inputTopic);
    }
  }

  $: {
    if (text !== "") {
      sessionStorage.setItem("text", text);
    }
  }

  onMount(() => {
    const inputTopicS = localStorage.getItem("inputTopic");

    if (inputTopicS) {
      inputTopic = inputTopicS;
    }

    const textS = sessionStorage.getItem("text");

    if (textS) {
      text = textS;
    }
  });

  /**
   * @param {string} topic
   */
  const isTopicValid = (topic) => {
    return !/[+#]/.test(topic) && !/^\/|\/$/.test(topic);
  };

  const autoTopic = () => {
    inputTopic = `${header.topic.replace(/[#+]/g, "$")}`;
  };

  const handleSubmit = async () => {
    if (submitLoading) {
      return;
    }

    submitLoading = true;
    //当type为Json 校验是否为Json格式
    if (type === "Json") {
      try {
        JSON.parse(text);
      } catch (e) {
        addToast({
          type: "red",
          message: "Json 格式错误",
          dismissible: true,
          timeout: 3000,
        });
        submitLoading = false;

        return;
      }
    }

    //校验topic是否为空
    if (inputTopic === "") {
      topicError = true;
      addToast({
        type: "red",
        message: "Topic 不能为空",
        dismissible: true,
        timeout: 3000,
      });
      submitLoading = false;

      return;
    }

    if (!isTopicValid(inputTopic)) {
      topicError = true;
      addToast({
        type: "red",
        message: "Topic 格式错误",
        dismissible: true,
        timeout: 3000,
      });
      submitLoading = false;

      return;
    }

    if (topicError) {
      setTimeout(() => {
        topicError = false;
      }, 1000);
    }

    const msg = {
      topic: inputTopic,
      qos: 1,
      ephemeral: false,
      type: type,
      raw: text,
    };

    sendMsg(msg).then(
      (/** @type {{ result: string; message: string; }} */ res) => {
        submitLoading = false;
        if (res.result === "error") {
          addToast({
            type: "red",
            message: "发送失败,错误原因：" + res.message,
            dismissible: true,
          });
        } else {
          submitSussess = true;
          text = "";
          setTimeout(() => {
            submitSussess = false;
          }, 1000);
        }
      }
    );
  };
</script>

<p class="hidden">{header}</p>
<div
  class="w-full h-full border-t border-gray-300 flex flex-col justify-end items-end bg-slate-200"
>
  <div
    class="w-full px-6 py-1 text-xl flex flex-row justify-between items-center"
  >
    <div class="flex gap-2 items-center">
      <div
        class="flex gap-1 items-center rounded-lg bg-white px-2 py-1 cursor-pointer"
      >
        <iconify-icon
          icon="streamline:mail-smiley-happy-face-chat-message-smiley-smile-emoji-face-satisfied"
        />
      </div>

      <div
        class="flex gap-1 items-center rounded-lg bg-white px-2 py-1 cursor-pointer"
        on:click={autoTopic}
        role="button"
        tabindex="0"
        class:hidden={!pattern.test(header.topic)}
        on:keydown={(e) => {}}
      >
        <iconify-icon icon="streamline:auto-flash" />
        <span class="text-sm">Auto Topic</span>
      </div>
    </div>

    <!-- Json MarkDown -->
    <EditorSwitch bind:value={type} />

    <!-- Qos -->
  </div>
  <div
    class="w-full px-6 py-2 text-xl flex flex-row justify-between items-center border-b border-gray-300 font-semibold"
  >
    <p class="text-sm">Topic:</p>
    <input
      type="text"
      class="border-2 outline-none text h-full flex-1 text-sm rounded-lg ml-2 focus:border-white focus:border-2"
      class:border-rose-600={topicError}
      class:border-white={!topicError}
      bind:value={inputTopic}
    />
  </div>
  <div class="w-full pl-6 pr-24 flex-1 py-2 relative">
    <textarea
      class="px-3 resize-none overflow-y-auto border-0 outline-none text rounded-lg w-full h-full"
      bind:value={text}
    />
    <div class="absolute bottom-2 right-4 h-full py-2">
      <Button
        class="w-16 m-2 h-full"
        on:click={handleSubmit}
        on:keydown={(e) => {
          e.preventDefault();
          if (e.key === "Enter" && e.ctrlKey) {
            console.log("submit");
            handleSubmit();
          }
        }}
      >
        {#if submitLoading && !submitSussess}
          <Spinner size="4" />
        {:else if submitSussess && !submitLoading}
          <iconify-icon icon="mdi:success-circle" />
        {:else}
          Submit
        {/if}
      </Button>
    </div>
  </div>
</div>

<style>
  .text {
    box-shadow: none;
  }
</style>
