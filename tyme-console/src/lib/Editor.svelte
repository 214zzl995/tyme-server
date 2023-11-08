<script>
  import Button from "flowbite-svelte/Button.svelte";
  import "iconify-icon";
  import EditorSwitch from "./EditorSwitch.svelte";
  import { addToast } from "./../js/store.js";
  import { sendMsg } from "./../js/fetch.js";

  export let topic = "tyme/1";

  let text = "";
  let type = "MarkDown";

  const handleSubmit = () => {
    //当type为Json 校验是否为Json格式
    if (type === "Json") {
      try {
        JSON.parse(text);
      } catch (e) {
        addToast({
          type: "red",
          message: "Json 格式错误",
          dismissible: true,
        });
        return;
      }
    }

    const msg = {
      topic: topic,
      qos: 1,
      mine: true,
      content: {
        type: type,
        raw: text,
      },
    };

    sendMsg(msg).then((/** @type {{ result: string; message: string; }} */ res) => {
      if (res.result === "error") {
        addToast({
          type: "red",
          message: "发送失败,错误原因：" + res.message,
          dismissible: true,
        });
      }
    });
  };
</script>

<div
  class="w-full h-full border-t border-gray-300 flex flex-col justify-end items-end"
>
  <div
    class="w-full h-8 px-6 text-xl flex flex-row justify-between items-center"
  >
    <iconify-icon
      class="cursor-pointer"
      icon="streamline:mail-smiley-happy-face-chat-message-smiley-smile-emoji-face-satisfied"
    />

    <!-- Json MarkDown -->
    <EditorSwitch bind:value={type} />

    <!-- Qos -->
  </div>
  <textarea
    class="px-6 flex-1 resize-none overflow-y-auto border-0 w-full outline-none text"
    bind:value={text}
  />
  <Button class="w-16 m-2" on:click={handleSubmit}>Submit</Button>
</div>

<style>
  .text {
    box-shadow: none;
  }
</style>
