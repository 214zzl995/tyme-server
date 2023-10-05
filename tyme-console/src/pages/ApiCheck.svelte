<script>
  import { getApi } from "../js/fetch.js";
  import Button from "flowbite-svelte/Button.svelte";
  import Label from "flowbite-svelte/Label.svelte";
  import Textarea from "flowbite-svelte/Textarea.svelte";

  let token = "1uY-1MPhNtIzQ1NK";
  let response = "";

  async function handlebutton() {
    response = "<nothing returned>";
    getApi(token).then((res) => {
      response = JSON.stringify(res);
    });
  }
</script>

<div class="w-11/12 sm:w-2/4 md:w-2/5 lg:w-2/4 p-8 bg-white rounded shadow-md">
  <container class="wider mobile">
    <p class="mb-6">
      You can try using the default API to access api of backend server or you
      can try entering a bad token to see what happens. This can be used with or
      without logging in.
    </p>

    <div class="grid grid-cols-1 gap-4 mb-6">
      <Label for="apiToken">API Token:</Label>
      <Textarea class="" type="text" bind:value={token} />
    </div>

    <Button class="mb-6" on:click={handlebutton}>Get /api</Button>

    <p class="mono">RESPONSE: {response}</p>
  </container>
</div>

<style>
</style>
