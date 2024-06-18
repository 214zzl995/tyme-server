<script>
  import { cubicInOut } from "svelte/easing";
  import { fade, fly } from "svelte/transition";

  export let open = false;
  export let title = "Modal";

  /**
   * @type {"sm" | "md" | "lg"}
   */
  export let size = "lg";
</script>

{#if open}
  <div
    class="w-screen h-screen bg-surface/30 backdrop-blur-md fixed z-[998] top-0 flex justify-center items-center"
    transition:fade={{
      duration: 300,
      easing: cubicInOut,
    }}
  >
    <button
      class="absolute top-4 right-4"
      on:click={() => {
        open = false;
      }}
    >
      <iconify-icon icon="ic:round-close" width="1.5em" height="1.5em" />
    </button>
    <div
      class:lg={size === "lg"}
      class:md={size === "md"}
      class:sm={size === "sm"}
      in:fly|local={{
        y: 100,
        duration: 300,
        easing: cubicInOut,
      }}
      class="bg-surface shadow-md rounded-lg p-8 w-full"
    >
      <div class="flex justify-between mb-10">
        <span class="font-semibold text-2xl">
          {title}
        </span>
      </div>

      <div>
        <slot></slot>
      </div>
    </div>
  </div>
{/if}

<style lang="postcss">
  .sm {
    @apply w-3/12;
  }

  .md {
    @apply w-6/12;
  }

  .lg {
    @apply w-9/12;
  }
</style>
