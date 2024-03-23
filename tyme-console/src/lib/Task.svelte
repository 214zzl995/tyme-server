<script>
  import Button from "flowbite-svelte/Button.svelte";
  import "iconify-icon";
  import { removeTask, restartTask, startTask, stopTask } from "../js/fetch.js";
  import { addToast } from "../js/store.js";
  import { createEventDispatcher } from "svelte";

  export let task;

  const dispatch = createEventDispatcher();

  const update = () => {
    dispatch("update");
  };

  const remove = () => {
    removeTask(task.id).then((res) => {
      if (res.result === "ok") {
        addToast({
          type: "green",
          message: "Remove Task Success.",
          dismissible: true,
          timeout: 3000,
        });
        dispatch("delete");
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

  const restart = () => {
    restartLoading = true;
    restartTask(task.id)
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
        restartLoading = false;
      });
  };

  let handleLoading = false;

  const start = () => {
    startTask(task.id)
      .then((res) => {
        if (res.result === "ok") {
          addToast({
            type: "green",
            message: "Start Task Success.",
            dismissible: true,
            timeout: 3000,
          });

          task.running = true;
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
        handleLoading = false;
      });
  };

  const stop = () => {
    stopTask(task.id)
      .then((res) => {
        if (res.result === "ok") {
          addToast({
            type: "green",
            message: "Stop Task Success.",
            dismissible: true,
            timeout: 3000,
          });

          task.running = false;
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
        handleLoading = false;
      });
  };

  const handleTask = () => {
    if (handleLoading) return;
    handleLoading = true;
    if (task.running) {
      stop();
    } else {
      start();
    }
  };

  const cronParser = () => {
    dispatch("cronParser");
  };
</script>

<div
  class="rounded-lg w-full h-48 shadow-lg p-3 flex flex-col bg-gradient-to-r from-purple-200 to-pink-200"
>
  <p class="text-xs h-6 flex flex-row items-center">
    <span
      class="block rounded-full p-1 w-16 flex justify-center items-center text-white mr-2 font-extrabold"
      class:bg-emerald-600={task.running}
      class:bg-pink-600={!task.running}
    >
      {#if task.running}
        已启动
      {:else}
        未启动
      {/if}</span
    >
    <span class="italic text-slate-400">
      {task.id}
    </span>
  </p>

  <p class="font-extrabold mt-2 text-4xl">
    {task.task.name}
  </p>

  <p class="flex flex-row-reverse py-3 flex-1 gap-2">
    <Button
      pill={true}
      size="xs"
      on:click={handleTask}
      class=" hover:underline dark:text-primary-500 !p-2 w-10 h-10"
    >
      {#if handleLoading}
        <iconify-icon
          style="color: #ffffff"
          icon="8-dots-rotate"
          height="1.2rem"
        />
      {:else if task.running}
        <iconify-icon
          style="color: #ffffff"
          icon="material-symbols:stop-circle-outline"
          height="1.2rem"
        />
      {:else}
        <iconify-icon
          style="color: #ffffff"
          icon="material-symbols:not-started-outline"
          height="1.2rem"
        />
      {/if}
    </Button>

    <Button
      pill={true}
      size="xs"
      on:click={restart}
      class="hover:underline dark:text-primary-500 !p-2 w-10 h-10"
    >
      {#if restartLoading}
        <iconify-icon
          style="color: #ffffff"
          icon="8-dots-rotate"
          height="1.2rem"
        />
      {:else}
        <iconify-icon
          style="color: #ffffff"
          icon="material-symbols:restart-alt"
          height="1.2rem"
        />
      {/if}
    </Button>
    <Button
      pill={true}
      size="xs"
      on:click={remove}
      class=" hover:underline dark:text-primary-500 !p-2 w-10 h-10"
    >
      <iconify-icon
        style="color: #ffffff"
        icon="material-symbols:delete-outline-rounded"
        height="1.2rem"
      />
    </Button>

    <Button
      pill={true}
      size="xs"
      on:click={update}
      class=" hover:underline dark:text-primary-500 !p-2 w-10 h-10"
    >
      <iconify-icon
        style="color: #ffffff"
        icon="material-symbols:edit-square-outline-rounded"
        height="1.2rem"
      />
    </Button>

    <Button
      pill={true}
      size="xs"
      on:click={cronParser}
      class=" hover:underline dark:text-primary-500 !p-2 w-10 h-10"
    >
      <iconify-icon
        style="color: #ffffff"
        icon="material-symbols:timer-5-outline"
        height="1.2rem"
      />
    </Button>
  </p>

  <div
    class="text-slate-600 mt-1 text-xs font-bold rounded bg-primary-700 px-2 py-2 mt-auto flex justify-between text-white"
  >
    <p>
      <span class="text-xs text-white rounded-full p-1 mr-2">
        {task.task.auto_start ? "自动启动" : "手动启动"}
      </span>
      <span>
        {task.task.cron}
      </span>
    </p>

    <span>
      {task.task.script}
    </span>
  </div>
</div>
