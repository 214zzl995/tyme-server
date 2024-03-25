<script>
  import { user } from "./../js/store.js";
  import { getSession, postLogin } from "./../js/auth";
  import Button from "flowbite-svelte/Button.svelte";
  import Label from "flowbite-svelte/Label.svelte";
  import Input from "flowbite-svelte/Input.svelte";
  import { createEventDispatcher, onMount } from "svelte";

  let username, password;
  let errorMessage = "";
  let passwordShow = false;

  const dispatch = createEventDispatcher();

  onMount(() => {});

  user.subscribe((value) => {
    if (value && value !== "") {
      dispatch("loginSuccess");
    }
  });

  async function handleLogin() {
    let loginResponse = await postLogin(username, password);
    if (loginResponse.result == "error") {
      errorMessage = loginResponse.message;
    } else {
      await getSession();
    }
  }
</script>

<div
  class="w-full h-[calc(100vh-4rem)] md:h-[calc(100vh-7rem)] flex items-center justify-center"
>
  <div
    class="w-11/12 sm:w-3/4 md:w-2/5 lg:w-2/4 xl:w-1/4 p-8 bg-stone-50 rounded bg-clip-padding p-6 border-4 border-violet-300 border-dashed mb-32"
  >
    <p class="text-3xl font-bold">Login</p>
    <div class="h-8 overflow-hidden">
      {#if errorMessage}
        <p class="text-rose-700 text-sm">
          {errorMessage}
        </p>
      {/if}
    </div>

    <container>
      <div>
        <Label class="text-xs text-slate-400 font-semibold">Username</Label>
        <Input
          type="text"
          placeholder="UserName"
          required
          bind:value={username}
        />
      </div>

      <div class="mt-4">
        <Label class="text-xs text-slate-400 font-semibold">Password</Label>
        <Input
          type={passwordShow ? "text" : "password"}
          placeholder="Password"
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

      <div class="mt-3 flex justify-end">
        <Button on:click={handleLogin}>Login</Button>
      </div>
    </container>
  </div>
</div>
