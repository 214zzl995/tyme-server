<script>
  import { onMount } from "svelte";
  import { user } from "../js/store.js";
  import { getSecure } from "../js/fetch.js";

  let response;

  onMount(async () => {
    getSecure().then((res) => {
      response = res.session;
    });
  });
</script>

<div class="w-11/12 sm:w-2/4 md:w-2/5 lg:w-2/4 p-8 bg-white rounded shadow-md mt-3">

</div>
