<script>
    import Toast from "flowbite-svelte/Toast.svelte";
    import "iconify-icon";
    import { dismissToast, toasts } from "./../js/store.js";

    const getIcon = (/** @type {string} */ type) => {
        switch (type) {
            case "red":
                return "ic:round-error";
            case "green":
                return "ic:round-check";
            case "blue":
                return "ic:round-info";
            case "yellow":
                return "ic:round-warning";
            default:
                return "ic:round-info";
        }
    };
</script>

{#if $toasts}
    <section
        class="mt-6 fixed w-full flex justify-center items-center flex-col z-[1000]"
    >
        {#each $toasts as toast (toast.id)}
            <Toast
                color={toast.type}
                dismissable={toast.dismissible}
                on:colse={(e) => {
                    dismissToast(toast.id);
                }}
                class="mb-3"
            >
                {toast.message}
                <svelte:fragment slot="icon">
                    <div class="w-5 h-5 flex justify-center items-center">
                        <iconify-icon
                            icon={getIcon(toast.type)}
                            class="font-medium"
                        />
                    </div>
                    <span class="sr-only">Check icon</span>
                </svelte:fragment>
            </Toast>
        {/each}
    </section>
{/if}
