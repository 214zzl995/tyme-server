<script>
  import NavBrand from "flowbite-svelte/NavBrand.svelte";
  import NavLi from "flowbite-svelte/NavLi.svelte";
  import NavUl from "flowbite-svelte/NavUl.svelte";
  import NavHamburger from "flowbite-svelte/NavHamburger.svelte";
  import Navbar from "flowbite-svelte/Navbar.svelte";
  import { onMount } from "svelte";

  export let navItems = [{ label: "logo", id: 0 }];
  export let menu = 1;

  let nowHash = "";

  let navBarHide;

  $: navBarHide = true;

  onMount(() => {
    nowHash = navItems.find((item) => item.id === menu)?.label;
  });

  const menuCilck = () => {
     navBarHide = !navBarHide;
  };

  const handleMenuSelection = (
    /** @type {number} */ id,
    /** @type {String} */ hash
  ) => {
    menu = id;
    nowHash = hash;
    navBarHide = true;
  };
</script>

<div class="w-full z-[999]">
  <Navbar class="px-2 sm:px-4 py-2.5 z-20  border-b ">
    <NavBrand href="/">
      <span
        class="self-center whitespace-nowrap text-xl font-semibold dark:text-white"
        >TYME</span
      >
    </NavBrand>
    <button on:click={menuCilck}>
      <NavHamburger />
    </button>

    <NavUl activeUrl="#{nowHash}" bind:hidden={navBarHide}>
      {#each navItems as item}
        <NavLi
          href="#{item.label}"
          on:click={() => handleMenuSelection(item.id, item.label)}
        >
          {item.label}
        </NavLi>
      {/each}
    </NavUl>
  </Navbar>
</div>

<style>
</style>
