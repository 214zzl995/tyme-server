<script>
  export let mine;
  export let source;
</script>

<div
  class="w-full flex p-3"
  class:flex-row={!mine}
  class:flex-row-reverse={mine}
>
  <div
    class="w-9/12 md:w-7/12 shadow-xl rounded-lg p-5 chat-card"
    class:bg-slate-400={!mine}
    class:bg-blue-400={mine}
  >
    <div>
      {@html source}
    </div>
  </div>
</div>

<style>
  .chat-card {
    overflow: auto;
  }

  :global(.chat-card *) {
    all: revert;
  }

  :global(.chat-card code) {
    background-color: dimgray;
    color: azure;
  }
</style>
