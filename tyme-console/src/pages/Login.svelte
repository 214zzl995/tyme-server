<script>
  import { user } from "./../js/store.js";
  import { getSession, postLogin } from "./../js/auth";
  import Button from "flowbite-svelte/Button.svelte";
  import Label from "flowbite-svelte/Label.svelte";
  import Input from "flowbite-svelte/Input.svelte";

  let username, password;
  let errorMessage = "";
  let passwordShow = false;

  async function handleLogin() {
    let loginResponse = await postLogin(username, password);
    if (loginResponse.result == "error") {
      errorMessage = loginResponse.message;
    } else {
      getSession();
    }
  }
</script>

{#if !$user}
  <div
    class="w-11/12 sm:w-2/4 md:w-2/5 lg:w-1/4 p-8 bg-white rounded shadow-md mt-3"
  >
    <div class="h-8 overflow-hidden">
      {#if errorMessage}
        <p class="text-rose-700 text-sm">
          {errorMessage}
        </p>
      {/if}
    </div>

    <container>
      <form class="grid gap-x-1 gap-y-3 mb-6 md:grid-cols-1">
        <div>
          <Label for="UserName" class="mb-2">Username</Label>
          <Input
            type="text"
            placeholder="UserName"
            required
            bind:value={username}
          />
        </div>

        <div>
          <Label for="PassWord" class="mb-2">PassWord</Label>
          <Input
            type={passwordShow ? "text" : "password"}
            placeholder="PassWord"
            required
            autocomplete
            bind:value={password}
          >
            <button
              class="outline-none border-none bg-transparent flex items-center justify-center"
              on:click={() => (passwordShow = !passwordShow)}
              slot="right"
            >
              <iconify-icon
                icon="ant-design:eye-invisible-twotone"
                class:hidden={passwordShow}
              />

              <iconify-icon
                icon="ant-design:eye-twotone"
                class:hidden={!passwordShow}
              />
            </button>
          </Input>
        </div>

        <Button on:click={handleLogin}>Login</Button>
      </form>
    </container>
  </div>
{:else}
  <div
    class="w-11/12 sm:w-2/4 md:w-2/5 lg:w-1/4 p-8 bg-white rounded shadow-md mt-3"
  >
    <div class="flex">
      <iconify-icon
        icon="mingcute:check-line"
        width="3rem"
        class="mb-3 rounded-full text-emerald-600 border-4 p-5"
      />

      <container class="ml-5">
        Logged in as: {$user} <br />
        Now you may access the <strong>secure area </strong>from the Nav above
      </container>
    </div>
  </div>
{/if}
