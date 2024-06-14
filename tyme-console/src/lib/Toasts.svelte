<script>
  import "iconify-icon";
  import { dismissToast, toasts } from "./../js/store.js";
  import { fly } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { cubicInOut, quintInOut, quintOut } from "svelte/easing";

  const getIcon = (/** @type {string} */ type) => {
    switch (type) {
      case "error":
        return "ic:round-error";
      case "primary":
        return "ic:round-check";
      case "secondary":
        return "ic:round-info";
      case "tertiary":
        return "ic:round-warning";
      default:
        return "ic:round-info";
    }
  };
</script>

{#if $toasts}
  {#each $toasts as toast, index (toast.id)}
    <div
      class="fixed right-4 z-[1000] flex flex-row items-center h-16 bottom-0 {toast.type} rounded-lg shadow-lg p-3 w-auto backdrop-blur-xl"
      style="bottom : calc(4.25rem * {index} + 2rem)"
      transition:fly={{ 
        x: 100, 
        duration: 300, 
        easing: cubicInOut
      }}
      animate:flip={{ duration: 300, easing: quintOut }}
    >
      <div
        class="mx-2 rounded-lg w-6 h-6 font-medium flex items-center justify-center bg-outline/50"
      >
        <iconify-icon icon={getIcon(toast.type)} />
      </div>
      <div><p class="w-auto">{toast.message}</p></div>
      <div>
        {#if toast.dismissible}
          <button
            class="rounded-md font-bold ml-16 mr-4 flex items-center justify-center"
            on:click={(e) => {
              dismissToast(toast.id);
            }}
          >
            <iconify-icon icon="ic:round-close" width="1.2em" height="1.2em" />
          </button>
        {/if}
      </div>
    </div>
  {/each}
{/if}

<style lang="postcss">
  .primary {
    @apply bg-primary text-on-primary;
  }
  .secondary {
    @apply bg-secondary text-on-secondary;
  }
  .tertiary {
    @apply bg-tertiary text-on-tertiary;
  }
  .error {
    @apply bg-error text-on-error;
  }
</style>
