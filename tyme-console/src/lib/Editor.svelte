<script>
  import Button from "flowbite-svelte/Button.svelte";
  import "iconify-icon";
  import EditorSwitch from "./EditorSwitch.svelte";

  let text = "";
  let type = "Json"

  

  const handleSubmit = () => {
    console.log(text);
  };
</script>

<div
  class="w-full h-full border-t border-gray-300 flex flex-col justify-end items-end"
>
  <div class="w-full h-8 px-6 text-xl flex flex-row justify-between items-center">
    <iconify-icon
      class="cursor-pointer"
      icon="streamline:mail-smiley-happy-face-chat-message-smiley-smile-emoji-face-satisfied"
    />

    <!-- Json MarkDown -->
    <EditorSwitch bind:value = {type}/>

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
