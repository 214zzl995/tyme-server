<script>
  import Label from "flowbite-svelte/Label.svelte";
  import Checkbox from "flowbite-svelte/Checkbox.svelte";
  import Input from "flowbite-svelte/Input.svelte";
  import GradientButton from "flowbite-svelte/GradientButton.svelte";
  import Textarea from "flowbite-svelte/Textarea.svelte";
  import Fileupload from "flowbite-svelte/Fileupload.svelte";
  import Helper from "flowbite-svelte/Helper.svelte";

  export let broker;
  export let port;
  export let clientId;
  export let keepAliveInterval;
  export let lwt;
  export let auth;
  export let ssl;
  export let crtFiles;

  let mqttAuthPasswordshow = false;

  let mqttAuthEnable;
  $: mqttAuthEnable = !auth.enable;

  let mqttSslEnable;
  $: mqttSslEnable = !ssl.enable;

  let crtName;
  $: crtName = ssl.trust_store.substr(ssl.trust_store.lastIndexOf("/") + 1);
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
        <Input bind:value={broker} class="max-w-none md:max-w-md" />
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
          <span class="text-red-600 hidden"> * </span>ClientId:
        </p>
      </Label>
      <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
        <Input bind:value={clientId} class="max-w-none md:max-w-md" />
      </div>
    </div>
    <div class="grid grid-cols-12 grid-rows-2 mt-2">
      <Label
        class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600 hidden"> * </span>KeepAliveInterval:
        </p>
      </Label>
      <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
        <Input
          type="number"
          placeholder="60"
          bind:value={keepAliveInterval}
          class="max-w-none md:max-w-md"
        />
      </div>
    </div>

    <div class="grid grid-cols-12 grid-rows-12 md:mt-2 mt-3">
      <Label
        class="col-span-12 md:col-span-4 row-span-2 md:row-span-12 block text-center flex"
      >
        <p class="my-auto">
          <span class="text-red-600"> * </span>Lwt Content:
        </p>
      </Label>
      <div class="col-span-12 md:col-span-8 row-span-10 md:row-span-12">
        <Textarea rows="8" bind:value={lwt} />
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
        />
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
        />
        <Helper
          >Current certificate: <span class="font-semibold">
            {crtName}</span
          ></Helper
        >
      </div>
    </div>
  </form>
  <slot />
</div>
