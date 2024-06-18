<script>
  import "iconify-icon";
  import { format } from "date-fns";
  import { PrimaryToast, ErrorToast } from "../js/store";

  export let msg;

  const showMsgNewTab = () => {
    let url = `\/c\/msg\/${encodeURIComponent(msg.id)}`;
    window.open(url, "_blank", "noopener,noreferrer");
  };

  const copyToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(msg.content.raw);
      PrimaryToast("Copy Success");
    } catch (error) {
      ErrorToast("Copy Failed");
    }
  };
</script>

<div class="p-2 w-full rounded-2xl">
  <div
    class="flex justify-between text-primary-container/50 dark:text-tertiary/50"
  >
    <p class="mb-2 text-xs">
      <span>
        {format(Date.parse(msg.timestamp), "yyyy-MM-dd HH:mm:ss")}
      </span>

      <span class="ml-2">
        Sender: <span class="font-semibold">{msg.sender}</span>
      </span>
      <span class="ml-2">
        Qos: <span class="font-semibold">{msg.qos}</span>
      </span>

      {#if msg.retain}
        <span class="font-semibold ml-2 bg-primary rounded-lg px-2">Retain</span
        >
      {/if}
      {#if msg.mine}
        <span class="font-semibold ml-2 bg-primary rounded-lg px-2">I Sent</span
        >
      {/if}
    </p>

    <div class="mr-2">
      <iconify-icon
        class="cursor-pointer"
        icon="line-md:text-box-to-text-box-multiple-transition"
        role="button"
        on:click={copyToClipboard}
        on:keydown={(/** @type {any} */ e) => {}}
        tabindex="0"
      />
      <iconify-icon
        class="cursor-pointer ml-2"
        icon="line-md:arrows-diagonal"
        on:click={showMsgNewTab}
        role="button"
        on:keydown={(/** @type {any} */ e) => {}}
        tabindex="0"
      />
    </div>
  </div>

  <div class="markdown-body p-3">
    {@html msg.content.html}
  </div>
</div>

<style lang="postcss">
</style>
