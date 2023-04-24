<script lang="ts">
    export let active = false;
    export let title: string;

    export const open = () => (active = true);
    export const close = () => (active = false);
</script>

{#if active}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div
        on:click={close}
        class="fixed left-0 top-0 z-10 flex h-screen w-screen items-center justify-center overflow-auto bg-black bg-opacity-50"
    >
        <div
            on:click|stopPropagation
            class="overflow-hidden rounded bg-white shadow-lg"
        >
            <div class="relative w-full bg-slate-200 p-1 text-center text-lg font-semibold">
                {title}
                <button
                    class="absolute top-0 bottom-0 right-2 text-slate-400 hover:text-black"
                    on:click={close}
                >
                    x
                </button>
            </div>

            <div class="p-4">
                <slot />
            </div>
        </div>
    </div>
{/if}
