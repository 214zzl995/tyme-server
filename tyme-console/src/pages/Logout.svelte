<script>
    import { user } from "./../js/store.js";
    import { getLogout } from "./../js/auth.js";
    import { onMount } from "svelte";
    import "iconify-icon";

    $user;

    onMount(getLogout);
    
</script>

<div class="w-11/12 sm:w-2/4 md:w-2/5 lg:w-1/4 p-8 bg-white rounded shadow-md mt-3">
    <container>
        {#if $user}
            <div class="flex">
                <iconify-icon
                    icon="mingcute:check-line"
                    width="3rem"
                    class="mb-3 rounded-full text-emerald-600 border-4 p-5"
                />
                <p class="text-lg font-bold">
                    You are still logged in as {$user}.
                </p>
            </div>
        {:else}
            <div class="flex">
                <iconify-icon
                    icon="solar:logout-linear"
                    width="3rem"
                    class="mb-3 rounded-full text-emerald-600 border-4 p-5"
                />
                <p class="text-lg font-bold">You are now logged out.</p>
            </div>
        {/if}
    </container>
</div>

<style>
    div {
        margin: 25px;
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
    }
</style>
