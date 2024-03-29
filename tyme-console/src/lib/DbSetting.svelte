<script>
    import Label from "flowbite-svelte/Label.svelte";
    import Input from "flowbite-svelte/Input.svelte";
    import Helper from "flowbite-svelte/Helper.svelte";

    export let database;

    let databaseError = false;
    let databaseErrorHelper = "DataBase is required";

    export const check = () => {
        return true;
    };
</script>

<div class="bg-white rounded shadow-md p-8 w-full mb-3">
    <form>
        <h1 class="font-bold text-2xl">DataBase</h1>

        <div class="grid grid-cols-12 grid-rows-2 mt-2">
            <Label
                class="col-span-12 md:col-span-4 row-span-1 md:row-span-2 block text-center flex"
            >
                <p class="my-auto">
                    <span class="text-red-600"> * </span>BaseUrl:
                </p>
            </Label>
            <div class="col-span-12 md:col-span-8 row-span-1 md:row-span-2">
                <Input
                    bind:value={database}
                    class="max-w-none md:max-w-md"
                    color={databaseError ? "red" : "base"}
                />
                {#if databaseError}
                    <Helper class="mt-1 h-4" color="red">
                        {databaseErrorHelper}
                    </Helper>
                {/if}
            </div>
        </div>
    </form>
    <slot />
</div>
