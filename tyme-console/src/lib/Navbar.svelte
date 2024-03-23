<script>
  import NavBrand from "flowbite-svelte/NavBrand.svelte";
  import NavLi from "flowbite-svelte/NavLi.svelte";
  import NavUl from "flowbite-svelte/NavUl.svelte";
  import NavHamburger from "flowbite-svelte/NavHamburger.svelte";
  import Navbar from "flowbite-svelte/Navbar.svelte";
  import appIco from "../assets/icons/app_ico.svg";
  import "iconify-icon";
  import { getLogout } from "./../js/auth.js";

  export let navItems = [{ label: "logo", id: 0 }];
  export let routerId;

  $: activeUrl = navItems.find((item) => item.id === routerId)?.label;
  $: navBarHide = true;

  const menuCilck = () => {
    navBarHide = !navBarHide;
  };

  const handleMenuSelection = async (
    /** @type {number} */ id,
    /** @type {String} */ hash,
  ) => {
    if (id === 99) {
      await getLogout();
      localStorage.removeItem("user");
      window.location.href = "/";
    } else {
      routerId = id;
      activeUrl = hash;
      navBarHide = true;
    }
  };
</script>

<div class="w-full z-[888] h-16 md:h-20 fixed top-0">
  <Navbar
    class="px-2 sm:px-4 py-2.5 z-20 border-b h-full backdrop-blur-xl bg-white/75"
    color="none"
  >
    <NavBrand href="/">
      <img src={appIco} alt="appIco" class="h-12 w-12 lg:h-16 lg:w-16" />
      <span
        class="self-center whitespace-nowrap text-2xl font-semibold dark:text-white"
        >TYME</span
      >
    </NavBrand>
    <button on:click={menuCilck}>
      <NavHamburger />
    </button>

    <NavUl activeUrl="#{activeUrl}" bind:hidden={navBarHide}>
      {#each navItems as item}
        <NavLi
          href="#{item.label}"
          on:click={() => handleMenuSelection(item.id, item.label)}
          class="{item.background || ''} {item.color || ''}"
        >
          <div class="flex flex-row justify-center items-center">
            {#if item.icon}
              <iconify-icon icon={item.icon} class="mr-2 font-semibold" />
            {/if}
            <span
              class="font-semibold hover:text-primary-500"
              class:text-primary-500={activeUrl === item.label}
              class:text-slate-500={activeUrl !== item.label}>{item.label}</span
            >
          </div>
        </NavLi>
      {/each}
    </NavUl>
  </Navbar>
</div>

<style>
</style>
