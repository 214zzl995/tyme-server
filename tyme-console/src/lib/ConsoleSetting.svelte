<script>
  import Label from "flowbite-svelte/Label.svelte";
  import Checkbox from "flowbite-svelte/Checkbox.svelte";
  import Input from "flowbite-svelte/Input.svelte";
  import GradientButton from "flowbite-svelte/GradientButton.svelte";

  export let publicNet;
  export let port;
  export let username;
  export let password;
  export let apiToken;

  let webConsolePasswordshow = false;
  let webConsoleTokenshow = false;
</script>

<div class="bg-white rounded shadow-md p-8 w-full mb-3">
  <form>
    <h1 class="font-bold text-2xl">WebConsole</h1>

    <div class="grid grid-cols-12 grid-rows-2 mt-2 h-12">
      <Label
        class="col-span-10 md:col-span-4 row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>Public Network:
        </p>
      </Label>
      <div
        class="col-span-2 md:col-span-8 row-span-2 flex items-center justify-end md:justify-start"
      >
        <Checkbox bind:checked={publicNet} />
      </div>
    </div>

    <div class="grid grid-cols-12 grid-rows-2 mt-2">
      <Label
        class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>Port:
        </p>
      </Label>
      <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
        <Input type="number" bind:value={port} class="max-w-none md:max-w-md" />
      </div>
    </div>

    <div class="grid grid-cols-12 grid-rows-2 mt-2">
      <Label
        class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>UserName:
        </p>
      </Label>
      <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
        <Input bind:value={username} class="max-w-none md:max-w-md" />
      </div>
    </div>

    <div class="grid grid-cols-12 grid-rows-2 mt-2">
      <Label
        class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>Password:
        </p>
      </Label>
      <div
        class="col-span-12 md:col-span-8 row-span-1 md:row-span-2 md:max-w-md"
      >
        <Input
          autocomplete="off"
          type={webConsolePasswordshow ? "text" : "password"}
          bind:value={password}
          class="max-w-none h-11"
        >
          <GradientButton
            slot="right"
            on:click={() => (webConsolePasswordshow = !webConsolePasswordshow)}
            class="pointer-events-auto flex items-centers"
          >
            <iconify-icon
              icon="ant-design:eye-invisible-twotone"
              class:hidden={webConsolePasswordshow}
            />

            <iconify-icon
              icon="ant-design:eye-twotone"
              class:hidden={!webConsolePasswordshow}
            />
          </GradientButton></Input
        >
      </div>
    </div>

    <div class="grid grid-cols-12 grid-rows-2 mt-2">
      <Label
        class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>API Token:
        </p>
      </Label>
      <div
        class="col-span-12 md:col-span-8 row-span-1 md:row-span-2 md:max-w-md"
      >
        <Input
          autocomplete="off"
          type={webConsoleTokenshow ? "text" : "password"}
          bind:value={apiToken}
          class="max-w-none h-11"
        >
          <GradientButton
            slot="right"
            on:click={() => (webConsoleTokenshow = !webConsoleTokenshow)}
            class="pointer-events-auto flex items-centers"
          >
            <iconify-icon
              icon="ant-design:eye-invisible-twotone"
              class:hidden={webConsoleTokenshow}
            />

            <iconify-icon
              icon="ant-design:eye-twotone"
              class:hidden={!webConsoleTokenshow}
            />
          </GradientButton></Input
        >
      </div>
    </div>
  </form>
</div>
