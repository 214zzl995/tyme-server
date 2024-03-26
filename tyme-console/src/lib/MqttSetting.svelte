<script>
  import Label from "flowbite-svelte/Label.svelte";
  import Checkbox from "flowbite-svelte/Checkbox.svelte";
  import Input from "flowbite-svelte/Input.svelte";
  import GradientButton from "flowbite-svelte/GradientButton.svelte";
  import Fileupload from "flowbite-svelte/Fileupload.svelte";
  import Helper from "flowbite-svelte/Helper.svelte";

  export let broker;
  export let port;
  export let clientId;
  export let keepAliveInterval;
  export let auth;
  export let ssl;
  export let crtFiles;

  let brokerError = false;
  let portError = false;
  let clientIdError = false;
  let keepAliveIntervalError = false;
  let authUsernameError = false;
  let authPasswordError = false;
  let crtError = false;

  let brokerErrorHelper = "Broker is required";
  let portErrorHelper = "Port is required";
  let clientIdErrorHelper = "ClientId is required";
  let keepAliveIntervalErrorHelper = "KeepAliveInterval is required";
  let authUsernameErrorHelper = "Auth Username is required";
  let authPasswordErrorHelper = "Auth Password is required";
  let crtErrorHelper = "Certificate is required";

  export const check = () => {
    if (broker === undefined || broker === "" || broker === null) {
      brokerError = true;
    } else {
      brokerError = false;
    }

    if (port === undefined || port === "" || port === null) {
      portError = true;
    } else {
      portError = false;
    }

    if (port < 1 || port > 65535) {
      portError = true;
      portErrorHelper = "Port must be greater than 0 and less than 65536";
    } else {
      portError = false;
    }

    if (clientId === undefined || clientId === "" || clientId === null) {
      clientIdError = true;
    } else {
      clientIdError = false;
    }

    if (auth.enable) {
      if (
        auth.username === undefined ||
        auth.username === "" ||
        auth.username === null
      ) {
        authUsernameError = true;
      } else {
        authUsernameError = false;
      }

      if (
        auth.password === undefined ||
        auth.password === "" ||
        auth.password === null
      ) {
        authPasswordError = true;
      } else {
        authPasswordError = false;
      }
    } else {
      authUsernameError = false;
      authPasswordError = false;
    }

    if (ssl.enable) {
      if (crtFiles === undefined || crtFiles.length === 0) {
        crtError = true;
      } else {
        crtError = false;
      }
    }else{
      crtError = false;
    }

    if (
      brokerError ||
      portError ||
      clientIdError ||
      keepAliveIntervalError ||
      authUsernameError ||
      authPasswordError ||
      crtError
    ) {
      return false;
    } else {
      return true;
    }
  };

  let mqttAuthPasswordshow = false;

  let mqttAuthEnable;
  $: mqttAuthEnable = !auth.enable;

  let mqttSslEnable;
  $: mqttSslEnable = !ssl.enable;

  let crtName;
  $: crtName =
    ssl.trust_store === null
      ? ""
      : ssl.trust_store.substr(ssl.trust_store.lastIndexOf("/") + 1);
</script>

<div class="w-full bg-white rounded shadow-md p-8 mb-3">
  <form>
    <h1 class="font-bold text-2xl">MQTT Config</h1>
    <div class="grid grid-cols-12 grid-rows-2 mt-2">
      <Label
        class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>Broker:
        </p>
      </Label>
      <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
        <Input
          bind:value={broker}
          class="max-w-none md:max-w-md"
          color={brokerError ? "red" : "base"}
        />

        {#if brokerError}
          <Helper class="mt-1 h-4" color="red">
            {brokerErrorHelper}
          </Helper>
        {/if}
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
          <span class="text-red-600"> * </span>ClientId:
        </p>
      </Label>
      <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
        <Input
          bind:value={clientId}
          class="max-w-none md:max-w-md"
          color={clientIdError ? "red" : "base"}
        />

        {#if clientIdError}
          <Helper class="mt-1 h-4" color="red">
            {clientIdErrorHelper}
          </Helper>
        {/if}
      </div>
    </div>
    <div class="grid grid-cols-12 grid-rows-2 mt-2">
      <Label
        class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600 hidden">   </span>KeepAliveInterval:
        </p>
      </Label>
      <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
        <Input
          type="number"
          placeholder="60"
          bind:value={keepAliveInterval}
          class="max-w-none md:max-w-md"
          color={keepAliveIntervalError ? "red" : "base"}
        />

        {#if keepAliveIntervalError}
          <Helper class="mt-1 h-4" color="red">
            {keepAliveIntervalErrorHelper}
          </Helper>
        {/if}
      </div>
    </div>

    <div class="grid grid-cols-12 grid-rows-2 mt-2 h-12">
      <Label
        class="col-span-10 md:col-span-4 row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>Auth Enable:
        </p>
      </Label>
      <div
        class="col-span-2 md:col-span-8 row-span-2 flex items-center justify-end md:justify-start"
      >
        <Checkbox bind:checked={auth.enable} />
      </div>
    </div>

    <div class="grid grid-cols-12 grid-rows-2 mt-2">
      <Label
        class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>Auth UserName:
        </p>
      </Label>
      <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
        <Input
          bind:disabled={mqttAuthEnable}
          bind:value={auth.username}
          class="max-w-none md:max-w-md"
          color={authUsernameError ? "red" : "base"}
        />

        {#if authUsernameError}
          <Helper class="mt-1 h-4" color="red">
            {authUsernameErrorHelper}
          </Helper>
        {/if}
      </div>
    </div>

    <div class="grid grid-cols-12 grid-rows-2 mt-2">
      <Label
        class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>Auth Password:
        </p>
      </Label>
      <div
        class="col-span-12 md:col-span-8 row-span-1 md:row-span-2 md:max-w-md"
      >
        <Input
          autocomplete="off"
          type={mqttAuthPasswordshow ? "text" : "password"}
          bind:disabled={mqttAuthEnable}
          bind:value={auth.password}
          class="max-w-none h-11"
          color={authPasswordError ? "red" : "base"}
        >
          <GradientButton
            slot="right"
            on:click={() => (mqttAuthPasswordshow = !mqttAuthPasswordshow)}
            color="purpleToBlue"
          >
            <iconify-icon
              icon="ant-design:eye-invisible-twotone"
              class:hidden={mqttAuthPasswordshow}
            />

            <iconify-icon
              icon="ant-design:eye-twotone"
              class:hidden={!mqttAuthPasswordshow}
            />
          </GradientButton>
        </Input>

        {#if authPasswordError}
          <Helper class="mt-1 h-4" color="red">
            {authPasswordErrorHelper}
          </Helper>
        {/if}
      </div>
    </div>

    <div class="grid grid-cols-12 grid-rows-2 mt-2 h-12">
      <Label
        class="col-span-10 md:col-span-4 row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>SSL Enable:
        </p>
      </Label>
      <div
        class="col-span-2 md:col-span-8 row-span-2 flex items-center justify-end md:justify-start"
      >
        <Checkbox bind:checked={ssl.enable} />
      </div>
    </div>

    <div class="grid grid-cols-12 grid-rows-3 mt-2">
      <Label
        class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>Upload Certificate:
        </p>
      </Label>
      <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-3">
        <Fileupload
          bind:disabled={mqttSslEnable}
          bind:files={crtFiles}
          accept=".crt"
          class="max-w-none md:max-w-md"
          color={crtError ? "red" : "base"}
        />
        <Helper
          >Current certificate: <span class="font-semibold">
            {crtName}</span
          ></Helper
        >

        {#if crtError}
          <Helper class="mt-1 h-4" color="red">
            {crtErrorHelper}
          </Helper>
        {/if}
      </div>
    </div>
  </form>
  <slot />
</div>
