<script>
  import "iconify-icon";
  import { format } from "date-fns";

  export let msg;

  $: mine = msg.mine;
  $: source = msg.content.html;

  const showMsgNewTab = () => {
    let url = `\/c\/msg\/${encodeURIComponent(msg.topic.header)}?id=${msg.id}`;
    window.open(url, "_blank", "noopener,noreferrer");
  };
</script>

<div
  class="w-full flex p-3"
  class:flex-row={!mine}
  class:flex-row-reverse={mine}
  id={msg.id}
>
  <div
    class="w-9/12 md:w-7/12 shadow-md rounded-lg px-2 md:px-5 py-1 chat-card"
    class:bg-stone-100={!mine}
    class:bg-blue-100={mine}
  >
    <div class="flex gap-px md:gap-3 md:flex-row flex-col text-xs">
      <span class="time">
        {format(msg.timestamp, "yyyy-MM-dd HH:mm:ss")}
      </span>
      <span class="topic">
        publish: {msg.publish}
      </span>
    </div>
    <div class="chat-card-md">
      {@html source}
    </div>
    <div class="flex flex-row text-slate-500 mb-1 justify-between text-sm">
      <p class="topic">
        {msg.topic.topic}
      </p>
      <div class="flex flex-row gap-2.5 items-center justify-end">
        <iconify-icon
          class="cursor-pointer"
          icon="akar-icons:enlarge"
          on:click={showMsgNewTab}
          role="button"
          on:keydown={(/** @type {any} */ e) => {}}
          tabindex="0"
        />

        <p class="text-right font-semibold">
          qos:{msg.qos}
        </p>
      </div>
    </div>
  </div>
</div>

<style lang="postcss">
  .chat-card .topic {
    color: transparent;
    transition: 0.5s ease;
  }

  .chat-card .time {
    --tw-text-opacity: 1;
    color: rgb(100 116 139 / var(--tw-text-opacity));
  }

  .chat-card:hover .topic {
    --tw-text-opacity: 1;
    color: rgb(100 116 139 / var(--tw-text-opacity));
  }

  :global(.chat-card-md) {
    margin-top: 0.25rem;
    overflow: auto;
  }
  :global(.chat-card-md *) {
    all: revert;
  }

  :global(.chat-card-md code) {
    background-color: rgb(44, 44, 44);
    display: block;
    color: rgb(241, 241, 241);
    border-radius: 0.5rem;
    padding: 0.5rem;
    overflow: auto;
  }

  :global(.chat-card-md th) {
    border: 1px rgba(0, 0, 0, 0.3) solid;
    padding: 1rem;
  }

  :global(.chat-card-md table) {
    max-width: 100%;
    border-collapse: collapse;
    overflow: auto;
  }

  :global(.chat-card-md td, th) {
    border: 1px rgba(0, 0, 0, 0.3) solid;
    padding: 1rem;
  }

  :global(.chat-card-md img) {
    width: 100%;
  }
</style>
