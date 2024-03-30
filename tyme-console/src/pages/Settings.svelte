<script>
  import GradientButton from "flowbite-svelte/GradientButton.svelte";
  import Spinner from "flowbite-svelte/Spinner.svelte";
  import "iconify-icon";
  import { onMount } from "svelte";
  import { getConfig, putConfig, uploadCrt } from "../js/fetch";
  import ConsoleSetting from "../lib/ConsoleSetting.svelte";
  import MqttSetting from "../lib/MqttSetting.svelte";
  import DbSetting from "../lib/DbSetting.svelte";

  let config;
  $: config = {
    mqtt_config: {
      auth: {},
      ssl: {
        trust_store: "./ssl/emqxsl-ca.crt",
      },
    },
    web_console_config: {},
  };

  let crtFiles;
  let saveLoading = false;
  let saveStatus = "";

  const saveConfig = () => {
    if (saveLoading == false) {
      saveLoading = !saveLoading;
    } else return;

    if (crtFiles !== undefined && crtFiles.length !== 0) {
      config.mqtt_config.ssl.trust_store = crtFiles[0].name;
      upLoadCrtFile();
    }

    config.mqtt_config.port = parseInt(config.mqtt_config.port);
    config.web_console_config.port = parseInt(config.web_console_config.port);
    config.mqtt_config.keep_alive_interval = parseInt(config.mqtt_config.keep_alive_interval);

    putConfig(config).then((res) => {
      if (res.result == "error") {
        saveStatus = "Save Failed";
      } else {
        saveStatus = "Save Success";
      }
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
      config = res;
    });
  });
</script>

<div
  class="fixed bottom-0 right-0 md:bottom-5 md:right-5 lg:right-10 lg:bottom-10
   z-[999] backdrop-blur-sm w-full md:w-auto h-30 md:h-auto p-5 md:p-0 flex flex-row-reverse"
>
  <GradientButton
    shadow
    class="w-none md:w-36 md:h-12"
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

<div class="w-11/12 sm:w-11/12 md:w-3/5 lg:w-2/4 mb-20 md:mb-3 mt-3 min-h-full">
  <DbSetting bind:database={config.database} />

  <MqttSetting
    bind:broker={config.mqtt_config.broker}
    bind:port={config.mqtt_config.port}
    bind:clientId={config.mqtt_config.client_id}
    bind:keepAliveInterval={config.mqtt_config.keep_alive_interval}
    bind:auth={config.mqtt_config.auth}
    bind:ssl={config.mqtt_config.ssl}
    bind:crtFiles
  />

  <ConsoleSetting
    bind:port={config.web_console_config.port}
    bind:username={config.web_console_config.username}
    bind:password={config.web_console_config.password}
    bind:apiToken={config.web_console_config.api_token}
  />
</div>
