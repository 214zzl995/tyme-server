<script>
  import Input from "flowbite-svelte/Input.svelte";
  import Label from "flowbite-svelte/Label.svelte";
  import Select from "flowbite-svelte/Select.svelte";
  import Textarea from "flowbite-svelte/Textarea.svelte";
  import Checkbox from "flowbite-svelte/Checkbox.svelte";
  import Fileupload from "flowbite-svelte/Fileupload.svelte";
  import Helper from "flowbite-svelte/Helper.svelte";
  import GradientButton from "flowbite-svelte/GradientButton.svelte";
  import Spinner from "flowbite-svelte/Spinner.svelte";
  import "iconify-icon";
  import { onMount } from "svelte";
  import { getConfig, putConfig, uploadCrt } from "../js/fetch";

  let qos = [
    {
      value: 0,
      name: "QOS0",
    },
    {
      value: 1,
      name: "QOS1",
    },
    {
      value: 2,
      name: "QOS2",
    },
  ];

  let lwtContentType = [
    {
      value: "MarkDown",
      name: "MarkDown",
    },
    {
      value: "Json",
      name: "Json",
    },
    {
      value: "Raw",
      name: "Raw",
    },
  ];

  let sysConfig;
  $: sysConfig = {
    mqtt_config: {
      lwt: {
        content: {},
      },
      auth: {},
      ssl: {
        trust_store: "./ssl/emqxsl-ca.crt",
      },
    },
    web_console_config: {},
  };

  let mqttAuthEnable;
  $: mqttAuthEnable = !sysConfig.mqtt_config.auth.enable;

  let mqttSslEnable;
  $: mqttSslEnable = !sysConfig.mqtt_config.ssl.enable;

  let crtName;
  $: crtName = sysConfig.mqtt_config.ssl.trust_store.substr(
    sysConfig.mqtt_config.ssl.trust_store.lastIndexOf("/") + 1
  );

  let crtFiles;

  let mqttAuthPasswordshow = false;
  let webConsolePasswordshow = false;
  let webConsoleTokenshow = false;

  let saveLoading = false;
  let saveStatus = "";

  const saveConfig = () => {
    if (saveLoading == false) {
      saveLoading = !saveLoading;
    } else return;

    if (crtFiles !== undefined && crtFiles.length !== 0) {
      sysConfig.mqtt_config.ssl.trust_store = "./ssl/" + crtFiles[0].name;
      upLoadCrtFile();
    }

    putConfig(sysConfig).finally(() => {
      console.log("更新配置成功");
      crtFiles = undefined;
      saveLoading = !saveLoading;
    });
  };

  const upLoadCrtFile = () => {
    const formData = new FormData();
    formData.append("file", crtFiles[0]);
    uploadCrt(crtFiles[0].name, formData).finally(() => {
      console.log("上传成功");
    });
  };

  onMount(() => {
    getConfig().then((res) => {
      sysConfig = res;
    });
  });
</script>

<div class="fixed bottom-5 right-5 z-[999] ">
  <GradientButton
    shadow
    class="w-none"
    color="purpleToBlue"
    on:click={saveConfig}
  >
    <iconify-icon icon="fluent:save-24-filled" class:hidden={saveLoading} />

    <div hidden={!saveLoading}>
      <Spinner class="mr-3" size="4" color="white" />
    </div>

    &nbsp
    <span>Save</span>
    <span hidden={saveStatus !== ""}>{saveStatus}</span>
  </GradientButton>
</div>

<div class="w-11/12 sm:w-11/12 md:w-3/5 lg:w-2/4 mb-3 mt-3">
  <div class="bg-white rounded shadow-md p-8 w-full mb-3">
    <form>
      <h1 class="font-bold text-2xl">System</h1>
      <div class="grid grid-cols-12 grid-rows-2 mt-2">
        <Label
          class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
        >
          <p class="my-auto">
            <span class="text-red-600"> * </span>Log location:
          </p>
        </Label>
        <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
          <Input
            bind:value={sysConfig.log_location}
            class="max-w-none md:max-w-md"
          />
        </div>
      </div>
    </form>
  </div>

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
            bind:value={sysConfig.mqtt_config.broker}
            class="max-w-none md:max-w-md"
          />
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
            bind:value={sysConfig.mqtt_config.port}
            class="max-w-none md:max-w-md"
          />
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
          <Input
            bind:value={sysConfig.mqtt_config.client_id}
            class="max-w-none md:max-w-md"
          />
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
            bind:value={sysConfig.mqtt_config.keep_alive_interval}
            class="max-w-none md:max-w-md"
          />
        </div>
      </div>
      <div class="grid grid-cols-12 grid-rows-2 mt-2">
        <Label
          class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
        >
          <p class="my-auto">
            <span class="text-red-600"> * </span>Lwt-Topic:
          </p>
        </Label>
        <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
          <Input
            placeholder="LWT"
            bind:value={sysConfig.mqtt_config.lwt.topic}
            class="max-w-none md:max-w-md"
          />
        </div>
      </div>

      <div class="grid grid-cols-12 grid-rows-2 mt-2">
        <Label
          class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
        >
          <p class="my-auto">
            <span class="text-red-600"> * </span>Lwt-Qos:
          </p>
        </Label>
        <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
          <Select items={qos} bind:value={sysConfig.mqtt_config.lwt.qos} />
        </div>
      </div>

      <div class="grid grid-cols-12 grid-rows-2 mt-2">
        <Label
          class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
        >
          <p class="my-auto">
            <span class="text-red-600"> * </span>Lwt Content Type:
          </p>
        </Label>
        <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
          <Select
            items={lwtContentType}
            bind:value={sysConfig.mqtt_config.lwt.content.type}
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
          <Textarea
            rows="8"
            bind:value={sysConfig.mqtt_config.lwt.content.text}
          />
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
          <Checkbox bind:checked={sysConfig.mqtt_config.auth.enable} />
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
            bind:value={sysConfig.mqtt_config.auth.user_name}
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
            bind:value={sysConfig.mqtt_config.auth.password}
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
          <Checkbox bind:checked={sysConfig.mqtt_config.ssl.enable} />
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
  </div>
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
          <Checkbox bind:checked={sysConfig.web_console_config.public} />
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
            bind:value={sysConfig.web_console_config.port}
            class="max-w-none md:max-w-md"
          />
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
            bind:value={sysConfig.web_console_config.user_name}
            class="max-w-none md:max-w-md"
          />
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
            bind:value={sysConfig.web_console_config.password}
            class="max-w-none h-11"
          >
            <GradientButton
              slot="right"
              on:click={() =>
                (webConsolePasswordshow = !webConsolePasswordshow)}
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
            bind:value={sysConfig.web_console_config.api_token}
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
</div>
