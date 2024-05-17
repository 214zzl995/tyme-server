<script>
  import { getSession, postLogin } from "./../js/auth";
  import { createEventDispatcher } from "svelte";

  let username = "",
    password = "";

  let errorMessage = "";
  let passwordShow = false;

  const dispatch = createEventDispatcher();

  async function handleLogin() {
    let loginResponse = await postLogin(username, password);
    if (username === "" || password === "") {
      errorMessage = "Please enter username and password";
    } else {
      if (loginResponse.result == "error") {
        errorMessage = loginResponse.message;
      } else {
        await getSession();
        dispatch("loginSuccess");
      }
    }
  }
</script>

<div
  class="w-full h-full flex items-center justify-center"
>
  <div
    class="w-11/12 sm:w-3/4 md:w-2/5 lg:w-2/4 xl:w-1/4 p-8 p-6 mb-48 md:mb-36 md:border-2 md:border-outline-variant rounded-xl"
  >
    <p
      class="text-4xl font-bold mb-2 flex flex-col gap-1 items-start md:mb-6 text-on-surface"
    >
      Login
    </p>
    <div class="h-8 overflow-hidden">
      {#if errorMessage}
        <p class="text-error text-sm">
          {errorMessage}
        </p>
      {/if}
    </div>

    <form class="w-full">
      <div class="w-full">
        <input
          type="text"
          name="username"
          class="w-full bg-secondary-container/50 text-on-secondary-container border-0 outline-none rounded-xl p-4 transition-all duration-300 ease-in-out"
          autocomplete="one-time-code"
          placeholder="Username"
          bind:value={username}
        />
      </div>

      <div class="mt-4 w-full relative">
        {#if passwordShow}
          <input
            type="text"
            name="password"
            class="w-full bg-secondary-container/50 text-on-secondary-container border-0 outline-none rounded-xl p-4 transition-all duration-300 ease-in-out"
            autocomplete="one-time-code"
            placeholder="Password"
            bind:value={password}
          />
        {:else}
          <input
            type="password"
            name="password"
            autocomplete="one-time-code"
            class="w-full bg-secondary-container/50 text-on-secondary-container border-0 outline-none rounded-xl p-4 transition-all duration-300 ease-in-out"
            placeholder="Password"
            bind:value={password}
          />
        {/if}
        <button
          on:click={() => {
            passwordShow = !passwordShow;
          }}
          type="button"
          class="absolute right-4 top-1/4 flex items-center justify-center bg-secondary text-on-secondary rounded-full w-8 h-8"
        >
          <iconify-icon
            icon="mi:eye-off"
            width="1.2em"
            height="1.2em"
            class:hidden={passwordShow}
          />

          <iconify-icon
            icon="mi:eye"
            width="1.2em"
            height="1.2em"
            class:hidden={!passwordShow}
          />
        </button>
      </div>

      <!-- 记住用户名 -->
      <div class="mt-3">
        <div class="flex items-center justify-center">
          <label class="cyberpunk-checkbox-label">
            <input type="checkbox" class="cyberpunk-checkbox" />
            Remember me</label
          >
        </div>
      </div>

      <div class="mt-4 md:mt-8 md:mb-3 flex justify-end">
        <button
          type="button"
          on:click={handleLogin}
          class="px-10 py-3 font-medium text-sm border-2
          border-primary-container
          hover:text-on-primary-container active:text-on-primary-container
          bg-surface
          hover:bg-primary-container active:bg-primary-container flex
          flex-row items-center justify-center gap-2 w-full rounded-full"
        >
          Login
        </button>
      </div>
    </form>
  </div>
</div>

<style lang="postcss">
  .cyberpunk-checkbox {
    @apply border-2 border-primary-container;
    appearance: none;
    width: 20px;
    height: 20px;
    border-radius: 5px;
    background-color: transparent;
    display: inline-block;
    position: relative;
    margin-right: 10px;
    cursor: pointer;
  }

  .cyberpunk-checkbox:before {
    @apply bg-primary;
    content: "";
    display: block;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%) scale(0);
    width: 10px;
    height: 10px;
    border-radius: 3px;
    transition: all 0.3s ease-in-out;
  }

  .cyberpunk-checkbox:checked:before {
    transform: translate(-50%, -50%) scale(1);
  }

  .cyberpunk-checkbox-label {
    @apply text-xs text-current;
    cursor: pointer;
    user-select: none;
    display: flex;
    align-items: center;
  }
</style>
