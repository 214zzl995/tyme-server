<script>
  import Label from "flowbite-svelte/Label.svelte";
  import Input from "flowbite-svelte/Input.svelte";
  import GradientButton from "flowbite-svelte/GradientButton.svelte";
  import Helper from "flowbite-svelte/Helper.svelte";

  export let port;
  export let username;
  export let password;
  export let apiToken;

  let portError = false;
  let usernameError = false;
  let passwordError = false;
  let apiTokenError = false;

  let portErrorHelper = "Port is required";
  let usernameErrorHelper = "UserName is required";
  let passwordErrorHelper = "Password is required";
  let apiTokenErrorHelper = "API Token is required";

  let webConsolePasswordshow = false;
  let webConsoleTokenshow = false;

  export const check = () => {
    if (port === undefined || port === "" || port === null) {
      portError = true;
      portErrorHelper = "Port is required";
    } else {
      portError = false;
    }

    if (username === undefined || username === "" || username === null) {
      usernameError = true;
      usernameErrorHelper = "UserName is required";
    } else {
      usernameError = false;
    }

    if (password === undefined || password === "" || password === null) {
      passwordError = true;
      passwordErrorHelper = "Password is required";
    } else {
      passwordError = false;
    }

    if (apiToken === undefined || apiToken === "" || apiToken === null) {
      apiTokenError = true;
      apiTokenErrorHelper = "API Token is required";
    } else {
      apiTokenError = false;
    }

    if (portError || usernameError || passwordError || apiTokenError) {
      return false;
    } else {
      return true;
    }
  };
</script>

<div class="bg-white rounded shadow-md p-8 w-full mb-3">
  <form>
    <h1 class="font-bold text-2xl">WebConsole</h1>

    <div class="grid grid-cols-12 grid-rows-2 mt-2">
      <Label
        class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>Port:
        </p>
      </Label>
      <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
        <Input
          type="number"
          bind:value={port}
          class="max-w-none md:max-w-md"
          color={portError ? "red" : "base"}
        />
        {#if portError}
          <Helper class="mt-1 h-4" color="red">
            {portErrorHelper}
          </Helper>
        {/if}
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
        <Input
          bind:value={username}
          class="max-w-none md:max-w-md"
          color={usernameError ? "red" : "base"}
        />
        {#if usernameError}
          <Helper class="mt-1 h-4" color="red">
            {usernameErrorHelper}
          </Helper>
        {/if}
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
          color={passwordError ? "red" : "base"}
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
        {#if passwordError}
          <Helper class="mt-1 h-4" color="red">
            {passwordErrorHelper}
          </Helper>
        {/if}
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
          color={apiTokenError ? "red" : "base"}
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
        {#if apiTokenError}
          <Helper class="mt-1 h-4" color="red">
            {apiTokenErrorHelper}
          </Helper>
        {/if}
      </div>
    </div>
  </form>

  <slot />
</div>
