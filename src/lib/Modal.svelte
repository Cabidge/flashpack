<script lang="ts">
    import { fade, fly } from 'svelte/transition';
    import { expoOut } from 'svelte/easing';

    export let active = false;
    export let title: string;

    export const open = () => (active = true);
    export const close = () => (active = false);
</script>

{#if active}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div
        on:click={close}
        transition:fade={{ duration: 100 }}
        class="fixed left-0 top-0 z-10 flex h-screen w-screen items-center justify-center overflow-auto bg-black bg-opacity-50"
    >
        <div
            on:click|stopPropagation
            transition:fly={{ y: 50, duration: 380, easing: expoOut }}
            class="rounded bg-white shadow-lg overflow-hidden"
        >
            <div class="relative p-1 font-semibold bg-slate-200 text-lg text-center w-full">
                {title}
                <button class="absolute top-0 bottom-0 right-2 text-slate-400 hover:text-black" on:click={close}> x </button>
            </div>

            <div class="p-4">
                <slot />
            </div>
        </div>
    </div>
{/if}
