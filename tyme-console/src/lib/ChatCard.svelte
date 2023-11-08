<script>
  export let msg;

  $: mine = msg.mine;
  $: source = msg.content.html;
</script>

<div
  class="w-full flex p-3"
  class:flex-row={!mine}
  class:flex-row-reverse={mine}
>
  <div
    class="w-9/12 md:w-7/12 shadow-md rounded-lg px-2 md:px-5 py-1"
    class:bg-slate-400={!mine}
    class:bg-blue-400={mine}
  >
    <div class="chat-card-md">
      {@html source}
    </div>
    <div class="mb-2 text-right font-semibold text-sm text-slate-500">
      <p>
        qos:{msg.qos}
      </p>
    </div>
  </div>
</div>

<style>
  :global(.chat-card-md) {
    margin-top: 0.25rem;
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
</style>
