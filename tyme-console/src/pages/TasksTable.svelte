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
    removeTask,
    restartTask,
  } from "../js/fetch.js";
  import Label from "flowbite-svelte/Label.svelte";
  import Input from "flowbite-svelte/Input.svelte";
  import Select from "flowbite-svelte/Select.svelte";
  import { addToast } from "../js/store.js";
  import Table from "flowbite-svelte/Table.svelte";
  import TableHead from "flowbite-svelte/TableHead.svelte";
  import TableHeadCell from "flowbite-svelte/TableHeadCell.svelte";
  import TableBody from "flowbite-svelte/TableBody.svelte";
  import TableBodyRow from "flowbite-svelte/TableBodyRow.svelte";
  import TableBodyCell from "flowbite-svelte/TableBodyCell.svelte";
  import { slide } from "svelte/transition";
  import Indicator from "flowbite-svelte/Indicator.svelte";

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

    addTask(task)
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
              task: task,
              running: task.auto_start,
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
        closeModel();
      });
  };

  let openRowId;

  const toggleRow = (/** @type {any} */ id) => {
    openRowId = openRowId === id ? null : id;
  };

  const removeTaskF = (/** @type {any} */ id) => {
    removeTask(id).then((res) => {
      if (res.result === "ok") {
        addToast({
          type: "green",
          message: "Remove Task Success.",
          dismissible: true,
          timeout: 3000,
        });
        tasks = tasks.filter((item) => item.id !== id);
      } else {
        addToast({
          type: "red",
          message: res.message,
          dismissible: true,
          timeout: 3000,
        });
      }
    });
  };

  let restartLoading;

  const restartTaskF = (/** @type {any} */ id) => {
    restartLoading = id;
    restartTask(id)
      .then((res) => {
        if (res.result === "ok") {
          addToast({
            type: "green",
            message: "Restart Task Success.",
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
      })
      .finally(() => {
        restartLoading = null;
      });
  };
</script>

<div
  class="w-11/12 xl:w-2/4 bg-white rounded shadow-md mt-3 h-[calc(100vh-7rem)] relative"
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
    <Table divClass="h-full w-full overscroll-contain overflow-auto">
      <TableHead defaultRow={false}>
        <tr class="table-row md:hidden">
          <TableHeadCell colspan="2" class="border-0">ID</TableHeadCell>
          <TableHeadCell colspan="1" class="border-0">Edit</TableHeadCell>
        </tr>
        <tr>
          <TableHeadCell class="border-0 w-2/12 md:w-3/12 hidden md:table-cell"
            >Id</TableHeadCell
          >
          <TableHeadCell class="border-0 w-5/12 md:w-2/12">Name</TableHeadCell>
          <TableHeadCell class="border-0 w-2/12 md:w-2/12">Cron</TableHeadCell>
          <TableHeadCell class="border-0 w-5/12 md:w-2/12">Script</TableHeadCell
          >
          <TableHeadCell class="border-0 hidden md:table-cell w-1/12"
            >Auto</TableHeadCell
          >
          <TableHeadCell class="border-0 hidden md:table-cell w-2/12">
            Edit
          </TableHeadCell>
        </tr>
      </TableHead>
      <TableBody tableBodyClass="divide-y-0">
        {#each tasks as taskWithId}
          <TableBodyRow
            on:click={() => toggleRow(taskWithId.id)}
            class="border-0"
          >
            <TableBodyCell colspan="2" tdClass="py-1 px-6 md:hidden">
              <div class="flex flex-row items-center gap-1">
                {#if restartLoading === taskWithId.id}
                  <Indicator color="blue" />
                {:else if restartLoading !== taskWithId.id && taskWithId.running}
                  <Indicator color="green" />
                {:else}
                  <Indicator color="red" />
                {/if}

                {taskWithId.id}
              </div>
            </TableBodyCell>
            <TableBodyCell
              tdClass="w-full md:w-auto py-1 px-6 md:py-4 hidden md:table-cell"
            >
              <div class="flex flex-row items-center gap-1">
                {#if restartLoading === taskWithId.id}
                  <Indicator color="blue" />
                {:else if restartLoading !== taskWithId.id && taskWithId.running}
                  <Indicator color="green" />
                {:else}
                  <Indicator color="red" />
                {/if}

                {taskWithId.id}
              </div>
            </TableBodyCell>
            <TableBodyCell tdClass="hidden md:table-cell py-4 px-6"
              >{taskWithId.task.name}</TableBodyCell
            >
            <TableBodyCell
              tdClass="hidden px-6 py-4 whitespace-nowrap md:table-cell"
              >{taskWithId.task.cron}</TableBodyCell
            >
            <TableBodyCell
              tdClass="hidden px-6 py-4 whitespace-nowrap md:table-cell"
              >{taskWithId.task.script}</TableBodyCell
            >
            <TableBodyCell tdClass="hidden px-6 py-4 md:table-cell"
              >{taskWithId.task.auto_start}</TableBodyCell
            >
            <TableBodyCell>
              <button
                on:click={() => {
                  removeTaskF(taskWithId.id);
                }}
                class="font-medium text-primary-600 hover:underline dark:text-primary-500"
                >Delete</button
              >
              <button
                on:click={() => {
                  restartTaskF(taskWithId.id);
                }}
                class="font-medium text-fuchsia-600 hover:underline dark:text-primary-500"
                >Restart</button
              >
            </TableBodyCell>
          </TableBodyRow>
          <TableBodyRow
            on:click={() => toggleRow(taskWithId.id)}
            class="md:hidden"
          >
            <TableBodyCell tdClass="px-6 py-4 whitespace-nowrap"
              >{taskWithId.task.name}</TableBodyCell
            >
            <TableBodyCell tdClass="px-6 py-4 whitespace-nowrap"
              >{taskWithId.task.cron}</TableBodyCell
            >
            <TableBodyCell tdClass="px-6 py-4 whitespace-nowrap"
              >{taskWithId.task.script}</TableBodyCell
            >
            <TableBodyCell tdClass="hidden px-6 py-4 md:table-cell"
              >{taskWithId.task.auto_start}</TableBodyCell
            >
          </TableBodyRow>
          {#if openRowId === taskWithId.id}
            <TableBodyRow color="custom">
              <TableBodyCell colspan="6" class="p-0">
                <div
                  class="px-7 py-3"
                  transition:slide={{ duration: 300, axis: "y" }}
                >
                  <iconify-icon
                    class="text-6xl"
                    icon="vscode-icons:file-type-taskfile"
                  ></iconify-icon>
                  <div class="font-medium flex flex-wrap gap-x-12 gap-y-4">
                    <p>
                      <span class="font-semibold">ID: </span>
                      {taskWithId.id}
                    </p>
                    <p>
                      <span class="font-semibold">Name: </span>
                      {taskWithId.task.name}
                    </p>
                    <p>
                      <span class="font-semibold">Cron: </span>
                      {taskWithId.task.cron}
                    </p>
                    <p>
                      <span class="font-semibold">Remark: </span>
                      {taskWithId.task.remark}
                    </p>
                    <p>
                      <span class="font-semibold">Script: </span>
                      {taskWithId.task.script}
                    </p>
                    <p>
                      <span class="font-semibold">Auto Start: </span>
                      {taskWithId.task.auto_start}
                    </p>
                  </div>
                </div>
              </TableBodyCell>
            </TableBodyRow>
          {/if}
        {/each}
      </TableBody>
    </Table>
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

    <div class="mb-6 flex flex-row items-center">
      <Label for="default-input" class="block"
        ><span class="text-red-700">* </span>Auto Start</Label
      >
      <label class="relative inline-flex items-center cursor-pointer h-7">
        <input
          class="sr-only peer"
          value=""
          type="checkbox"
          bind:checked={task.auto_start}
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
          <Button on:click={addTaskF}>OK</Button>
          <Button on:click={closeModel} color="alternative">Cancel</Button>
        </div>
      </div>
    </svelte:fragment>
  </Modal>
</div>
