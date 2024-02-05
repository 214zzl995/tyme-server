<script>
  import { onMount } from "svelte";
  import Button from "flowbite-svelte/Button.svelte";
  import "iconify-icon";
  import Modal from "flowbite-svelte/Modal.svelte";
  import { getAllTask } from "../js/fetch.js";
  import { addToast } from "../js/store";

  let addModal = false;

  let tasks = [];

  let error = "";

  onMount(async () => {
    getAllTask().then((res) => {
      if (res.result === "ok") {
        error = "";
        tasks = res.tasks;
      } else {
        error = res.message;
      }
    });
  });
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

      <Button pill href="/c/sys.lua">
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
    autoclose
    outsideclose
  >
    <p>任务名称</p>
    <p>cron表达式</p>
    <p>任务描述</p>
    <p>选择lua脚本 右侧有上载按钮</p>
    <p>文件预上传 直接显示个是否存在 存在会覆盖提醒 确认 直接加任务</p>
    <svelte:fragment slot="footer">
      <Button on:click={() => alert('Handle "success"')}>OK</Button>
      <Button color="alternative">Cancel</Button>
    </svelte:fragment>
  </Modal>
</div>
