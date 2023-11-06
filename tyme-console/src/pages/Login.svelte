<script>
  import { user } from "./../js/store.js";
  import { getSession, postLogin } from "./../js/auth";
  import Button from "flowbite-svelte/Button.svelte";
  import Label from "flowbite-svelte/Label.svelte";
  import Input from "flowbite-svelte/Input.svelte";

  let username, password;
  let errorMessage = "";

  async function handleLogin() {
    let loginResponse = await postLogin(username, password);
    errorMessage = loginResponse.message;
    if (loginResponse.result == "error") {
      errorMessage = loginResponse.message;
    } else {
      getSession();
    }
  }
</script>

{#if !$user}
  <div
    class="w-11/12 sm:w-2/4 md:w-2/5 lg:w-1/4 p-8 bg-white rounded shadow-md"
  >
    {#if errorMessage}
      <div>
        {errorMessage}
      </div>
    {/if}
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
            type="password"
            placeholder="PassWord"
            required
            autocomplete
            bind:value={password}
          />
        </div>

        <Button on:click={handleLogin}>Login</Button>
      </form>
    </container>
  </div>
{:else}
  <div>
    <container>
      Logged in as: {$user} <br />
      Now you may access the <strong>secure area </strong>from the Nav above
    </container>
  </div>
{/if}

<style>
</style>
