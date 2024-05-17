<script>
  import "iconify-icon";
  import { getLogout } from "./../js/auth.js";

  export let navItems = [{ label: "logo", id: 0 }];
  export let routerId;

  $: activeUrl = navItems.find((item) => item.id === routerId)?.label;
  $: navBarHide = true;

  const handleMenuSelection = async (
    /** @type {number} */ id,
    /** @type {String} */ hash
  ) => {
    routerId = id;
    activeUrl = hash;
    navBarHide = true;
  };

  const Logout = async () => {
    await getLogout();
    localStorage.removeItem("user");
    window.location.href = "/";
  };
</script>

<nav
  class="px-2 sm:px-4 py-2.5 z-20 h-full backdrop-blur-xl bg-surface"
>
  <ul class="flex gap-2 justify-start flex-col h-full">
    {#each navItems as item}
      <li>
        <button
          type="button"
          class="rounded-full p-2 w-full"
          on:click={() => handleMenuSelection(item.id, item.label)}
        >
          {item.label}
        </button>
      </li>
    {/each}

    <li class="flex-1 relative">
      <button
        type="button"
        class="rounded-full p-2 absolute bottom-0 w-full"
        on:click={Logout}
      >
        Logout
      </button>
    </li>
  </ul>
</nav>

<style lang="postcss">
</style>
