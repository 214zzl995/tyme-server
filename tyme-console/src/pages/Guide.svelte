<script>
  import appIco from "../assets/icons/app_ico.svg";
  import ConsoleSetting from "../lib/ConsoleSetting.svelte";
  import Button from "flowbite-svelte/Button.svelte";
  import MqttSetting from "../lib/MqttSetting.svelte";
  import { getConfigGuide, putConfigGuide, uploadCrtGuide } from "../js/fetch";
  import { onMount } from "svelte";

  let crtFiles;

  let config = {
    mqtt_config: {
      broker: "k37bbe35.ala.cn-hangzhou.emqxsl.cn",
      port: 8883,
      client_id: "test",
      keep_alive_interval: 60,
      topics: [
        {
          topic: "command/#",
          qos: 1,
        },
        {
          topic: "chat/#",
          qos: 1,
        },
      ],
      version: 5,
      lwt: "Im offline",
      auth: {
        enable: true,
        username: "leri",
        password: "R7ddsQxAGchQPQB",
      },
      ssl: {
        enable: true,
        trust_store: "./ssl/emqxsl-ca.crt",
        key_store: null,
        private_key: null,
        private_key_password: null,
        ca_path: null,
        protos: null,
      },
    },
    web_console_config: {
      public: true,
      username: "root",
      password: "FxL3nw70",
      port: 12566,
      api_token: "1uY-1MPhNtIzQ1NK",
    },
  };

  let activeTab = 0;
  let saveLoading = false;
  let saveStatus = "";

  const saveConfig = () => {
    if (saveLoading == false) {
      saveLoading = !saveLoading;
    } else return;

    if (crtFiles !== undefined && crtFiles.length !== 0) {
      config.mqtt_config.ssl.trust_store = "./ssl/" + crtFiles[0].name;
      upLoadCrtFile();
    }

    putConfigGuide(config).then((res) => {
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
    uploadCrtGuide(crtFiles[0].name, formData).finally(() => {
      console.log("上传成功");
    });
  };

  onMount(() => {
    getConfigGuide().then((res) => {
      config = res;
    });
  });
</script>

<div
  class="min-h-screen bg-gradient-to-r from-cyan-100 to-blue-100 flex items-center flex-col md:flex-row"
>
  <div
    class="w-full h-32 md:h-full md:w-2/5 flex justify-center flex-col items-center sticky top-0 backdrop-blur-md md:relative"
  >
    <p class="flex">
      <img src={appIco} alt="appIco" class="h-24 w-24" />
      <span class="font-black text-[3.5rem] h-auto">Tyme</span>
    </p>
  </div>
  <div
    class="flex-1 md:h-full w-11/12 md:w-3/5 flex justify-center items-center mt-3 overflow-x-hidden px-2 md:px-12 lg:px-24"
  >
    <div class="w-full whitespace-nowrap">
      <div class="w-full inline-block">
        {#if activeTab === 0}
          <ConsoleSetting
            bind:publicNet={config.web_console_config.public}
            bind:port={config.web_console_config.port}
            bind:username={config.web_console_config.username}
            bind:password={config.web_console_config.password}
            bind:apiToken={config.web_console_config.api_token}
          >
            <div class="w-full flex flex-row justify-end gap-2 mt-6">
              <Button
                class="hidden"
                color="alternative"
                disabled={activeTab == 0}>Previous</Button
              >
              <Button on:click={() => (activeTab = 1)}>Next</Button>
            </div>
          </ConsoleSetting>
        {:else}
          <MqttSetting
            bind:broker={config.mqtt_config.broker}
            bind:port={config.mqtt_config.port}
            bind:clientId={config.mqtt_config.client_id}
            bind:keepAliveInterval={config.mqtt_config.keep_alive_interval}
            bind:lwt={config.mqtt_config.lwt}
            bind:auth={config.mqtt_config.auth}
            bind:ssl={config.mqtt_config.ssl}
            bind:crtFiles
          >
            <div class="w-full flex flex-row justify-end gap-2 mt-6">
              <Button color="alternative" on:click={() => (activeTab = 0)}
                >Previous</Button
              >
              <Button on:click={saveConfig}>Confirm</Button>
            </div>
          </MqttSetting>
        {/if}
      </div>
    </div>
  </div>
</div>
