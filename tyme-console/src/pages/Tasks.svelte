<script>
  import { onMount } from "svelte";
  import Button from "flowbite-svelte/Button.svelte";
  import "iconify-icon";
  import Modal from "flowbite-svelte/Modal.svelte";
  import { getAllScriptFile, uploadScript, addTask, getAllTask } from "../js/fetch.js";
  import Label from "flowbite-svelte/Label.svelte";
  import Input from "flowbite-svelte/Input.svelte";
  import Select from "flowbite-svelte/Select.svelte";
  import { addToast } from "../js/store.js";

  let addModal = false;

  let scriptFiles = [];
  let tasks = [];

  let error = "";

  let uploadScriptFile;

  onMount(async () => {
    getAllScriptFile().then((res) => {
      if (res.result === "ok") {
        error = "";
        scriptFiles = res.scripts;
      } else {
        error = res.message;
      }
    });

    getAllTask().then((res) => {
      if (res.result === "ok") {
        tasks = res.tasks;
      } else {
        error = res.message;
      }
    });
  });

  $: if (uploadScriptFile !== undefined && uploadScriptFile.length > 0) {
    if (uploadScriptFile[0].name === "sys.lua") {
      addToast({
        type: "red",
        message: "sys.lua is a system file, please do not upload it.",
        dismissible: true,
        timeout: 3000,
      });
    }

    const formData = new FormData();
    formData.append("file", uploadScriptFile[0]);
    uploadScript(uploadScriptFile[0].name, formData).then(() => {
      const name = uploadScriptFile[0].name;
      uploadScriptFile = undefined;
      if (!scriptFiles.find((item) => item.name === name)) {
        scriptFiles = [
          ...scriptFiles,
          {
            name: name,
            value: name,
          },
        ];
      }

      addToast({
        type: "green",
        message: name + "上传成功",
        dismissible: true,
        timeout: 3000,
      });
    });
  }

  let task = {
    name: "",
    cron: "",
    remark: "",
    script: "",
    auto_start: true,
  };

  const closeModel = () => {
    task = {
      name: "",
      cron: "",
      remark: "",
      script: "",
      auto_start: true,
    };
    addModal = false;
  };

  const addTaskF = () => {
    if (task.name === "") {
      addToast({
        type: "red",
        message: "Task Name is required.",
        dismissible: true,
        timeout: 3000,
      });
      return;
    }

    if (task.cron === "") {
      addToast({
        type: "red",
        message: "Cron is required.",
        dismissible: true,
        timeout: 3000,
      });
      return;
    }

    if (task.script === "") {
      addToast({
        type: "red",
        message: "Script is required.",
        dismissible: true,
        timeout: 3000,
      });
      return;
    }

    addTask(task).then((res) => {
      if (res.result === "ok") {
        addToast({
          type: "green",
          message: "Add Task Success.",
          dismissible: true,
          timeout: 3000,
        });
      } else {
        addToast({
          type: "red",
          message: res.message,
          dismissible: true,
          timeout: 3000,
        });
      }
    });

    closeModel();
  };
</script>

<div
  class="w-11/12 sm:w-2/4 md:w-2/5 lg:w-2/4 bg-white rounded shadow-md mt-3 h-[calc(100vh-7rem)] relative"
>
  <div>
    <div
      class="h-16 absolute top-0 left-0 w-full border-b flex flex-row-reverse items-center px-3 gap-3"
    >
      <Button pill on:click={() => (addModal = true)}>
        <iconify-icon icon="mingcute:add-fill" class="mr-2"></iconify-icon>
        Add
      </Button>

      <Button pill href="/c/script-file/sys.lua" download={"sys.lua"}>
        <iconify-icon icon="pajamas:download" class="mr-2"></iconify-icon>
        SDK
      </Button>
    </div>

    <div>
      <!-- 所有任务 -->
    </div>
  </div>

  <Modal
    title="Add Task"
    backdropClass="z-[998] fixed inset-0 bg-gray-900 bg-opacity-50 dark:bg-opacity-80"
    dialogClass="z-[999] fixed top-0 start-0 end-0 h-modal md:inset-0 md:h-full w-full p-4 flex"
    bind:open={addModal}
    outsideclose
  >
    <div class="mb-6">
      <Label for="default-input" class="block mb-2"
        ><span class="text-red-700">* </span>Task Name</Label
      >
      <Input
        id="default-input"
        placeholder="Task Name"
        bind:value={task.name}
      />
    </div>
    <div class="mb-6">
      <Label for="default-input" class="block mb-2"
        ><span class="text-red-700">* </span>Cron</Label
      >
      <Input id="default-input" placeholder="Cron" bind:value={task.cron} />
    </div>
    <div class="mb-6">
      <Label for="default-input" class="block mb-2"
        ><span class="text-red-700"> </span>Remark</Label
      >
      <Input id="default-input" placeholder="Remark" bind:value={task.remark} />
    </div>

    <div class="mb-6">
      <Label for="default-input" class="block mb-2"
        ><span class="text-red-700">* </span>Script</Label
      >
      <div class="flex flex-row">
        <Select class="mt-2" items={scriptFiles} bind:value={task.script} />
      </div>
    </div>

    <svelte:fragment slot="footer">
      <div class="flex w-full">
        <div class="flex-1">
          <input
            type="file"
            id="uploadScriptEl"
            style="display:none;"
            bind:files={uploadScriptFile}
            accept=".lua"
          />
          <Button
            on:click={() => {
              document.getElementById("uploadScriptEl").click();
              return false;
            }}
          >
            <iconify-icon icon="iconamoon:cloud-upload-bold" class="mr-2"
            ></iconify-icon>
            Up Load Script</Button
          >
        </div>
        <div class="flex-row-reverse">
          <Button on:click={addTaskF}>OK</Button>
          <Button on:click={closeModel} color="alternative">Cancel</Button>
        </div>
      </div>
    </svelte:fragment>
  </Modal>
</div>
