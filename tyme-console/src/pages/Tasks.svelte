<script>
  import { onMount } from "svelte";
  import Button from "flowbite-svelte/Button.svelte";
  import "iconify-icon";
  import Modal from "flowbite-svelte/Modal.svelte";
  import {
    getAllScriptFile,
    uploadScript,
    addTask,
    getAllTask,
    updateTask,
  } from "../js/fetch.js";
  import Label from "flowbite-svelte/Label.svelte";
  import Input from "flowbite-svelte/Input.svelte";
  import Select from "flowbite-svelte/Select.svelte";
  import { addToast } from "../js/store.js";
  import Task from "../lib/Task.svelte";
  import cronParser from "cron-parser";

  let addModal = false;
  let cronParserModal = false;

  let scriptFiles = [];
  let tasks = [];

  let error = "";

  let uploadScriptFile;

  let isEdit = false;

  let editId = "";

  let modelTitle = "Add Task";

  let cronModelContent = [];

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

    uploadScript(uploadScriptFile[0].name, uploadScriptFile[0]).then(() => {
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

  let modelTask = {
    id: "",
    name: "",
    cron: "",
    remark: "",
    script: "",
    auto_start: true,
  };

  const modelClose = () => {
    modelTask = {
      id: "",
      name: "",
      cron: "",
      remark: "",
      script: "",
      auto_start: true,
    };
    addModal = false;
  };

  const modelConfirm = () => {
    if (modelTask.name === "") {
      addToast({
        type: "red",
        message: "Task Name is required.",
        dismissible: true,
        timeout: 3000,
      });
      return;
    }

    if (modelTask.cron === "") {
      addToast({
        type: "red",
        message: "Cron is required.",
        dismissible: true,
        timeout: 3000,
      });
      return;
    }

    if (modelTask.script === "") {
      addToast({
        type: "red",
        message: "Script is required.",
        dismissible: true,
        timeout: 3000,
      });
      return;
    }
    if (isEdit) {
      updateTask(editId, modelTask)
        .then((res) => {
          if (res.result === "ok") {
            addToast({
              type: "green",
              message: "Edit Task Success.",
              dismissible: true,
              timeout: 3000,
            });
            tasks = tasks.map((item) => {
              if (item.id === editId) {
                item.task = modelTask;
              }
              return item;
            });
          } else {
            addToast({
              type: "red",
              message: res.message,
              dismissible: true,
              timeout: 3000,
            });
          }
        })
        .finally(() => {
          modelClose();
          editId = "";
        });
    } else {
      addTask(modelTask)
        .then((res) => {
          if (res.result === "ok") {
            addToast({
              type: "green",
              message: "Add Task Success.",
              dismissible: true,
              timeout: 3000,
            });
            let id = res.id;
            tasks = [
              {
                id: id,
                task: modelTask,
                running: modelTask.auto_start,
              },
              ...tasks,
            ];
          } else {
            addToast({
              type: "red",
              message: res.message,
              dismissible: true,
              timeout: 3000,
            });
          }
        })
        .finally(() => {
          modelClose();
        });
    }
  };

  const deleteTask = (/** @type {any} */ id) => {
    tasks = tasks.filter((item) => item.id !== id);
  };

  const updateTaskHandle = (/** @type {any} */ runner) => {
    isEdit = true;
    modelTask = runner.task;
    editId = runner.task.id;
    modelTitle = "Edit Task: " + runner.task.name;
    addModal = true;
  };

  const cronParserHandle = (/** @type {any} */ cron) => {
    try {
      let interval = cronParser.parseExpression(cron, {
        tz: "Asia/Shanghai",
      });
      for (let i = 0; i < 5; i++) {
        cronModelContent[i] = interval.next().toString();
      }
    } catch (e) {
      addToast({
        type: "red",
        message: e.message,
        dismissible: true,
        timeout: 3000,
      });
    }
    cronParserModal = true;
  };
</script>

<div
  class="w-11/12 xl:w-3/4 bg-white rounded shadow-md mt-3 h-[calc(100vh-7rem)] relative"
>
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

  <div class="mt-16 w-full h-[calc(100vh-11rem)] absolute">
    <div class="grid grid-cols-1 md:grid-cols-3 p-2 gap-2">
      {#each tasks as runner}
        <Task
          {runner}
          on:delete={() => deleteTask(runner.task.id)}
          on:update={() => updateTaskHandle(runner)}
          on:cronParser={() => cronParserHandle(runner.task.cron)}
        />
      {/each}
    </div>
  </div>

  <Modal
    title={modelTitle}
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
        bind:value={modelTask.name}
      />
    </div>
    <div class="mb-6">
      <Label for="default-input" class="block mb-2"
        ><span class="text-red-700">* </span>Cron</Label
      >
      <Input
        id="default-input"
        placeholder="Cron"
        bind:value={modelTask.cron}
      />
    </div>
    <div class="mb-6 hidden">
      <Label for="default-input" class="block mb-2"
        ><span class="text-red-700"> </span>Remark</Label
      >
      <Input
        id="default-input"
        placeholder="Remark"
        bind:value={modelTask.remark}
      />
    </div>

    <div class="mb-6">
      <Label for="default-input" class="block mb-2"
        ><span class="text-red-700">* </span>Script</Label
      >
      <div class="flex flex-row">
        <Select
          class="mt-2"
          items={scriptFiles}
          bind:value={modelTask.script}
        />
      </div>
    </div>

    <div class="mb-6 flex flex-row items-center">
      <Label for="default-input" class="block"
        ><span class="text-red-700">* </span>Auto Start</Label
      >
      <label class="relative inline-flex items-center cursor-pointer h-7">
        <input
          class="sr-only peer"
          value=""
          type="checkbox"
          bind:checked={modelTask.auto_start}
        />
        <div
          class="scale-50 peer rounded-full outline-none duration-100 after:duration-500 w-28 h-14 bg-blue-300 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-500 after:content-['No'] after:absolute after:outline-none after:rounded-full after:h-12 after:w-12 after:bg-white after:top-1 after:left-1 after:flex after:justify-center after:items-center after:text-sky-800 after:font-bold peer-checked:after:translate-x-14 peer-checked:after:content-['Yes'] peer-checked:after:border-white"
        ></div>
      </label>
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
          <Button on:click={modelConfirm}>OK</Button>
          <Button on:click={modelClose} color="alternative">Cancel</Button>
        </div>
      </div>
    </svelte:fragment>
  </Modal>

  <Modal title="Cron Parser" bind:open={cronParserModal} autoclose>
    <div class="p-2">
      <p class="text-sm text-gray-500 mb-2">
        The next 5 times of the cron expression are as follows:
      </p>
      <ul class="list-disc list-inside">
        {#each cronModelContent as item}
          <li>{item}</li>
        {/each}
      </ul>
    </div>
    <svelte:fragment slot="footer"></svelte:fragment>
  </Modal>
</div>
