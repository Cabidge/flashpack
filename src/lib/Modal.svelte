<script>
    import { fade, fly } from 'svelte/transition';
    import { expoOut } from 'svelte/easing';

    export let active = false;
</script>

{#if active}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div
        on:click={() => (active = false)}
        transition:fade={{ duration: 100 }}
        class="fixed z-10 left-0 top-0 w-screen h-screen overflow-auto bg-black bg-opacity-50 flex items-center justify-center"
    >
        <div
            on:click|stopPropagation
            transition:fly={{ y: 50, duration: 380, easing: expoOut }}
            class="relative bg-white shadow-lg p-6 rounded"
        >
            <button
                class="absolute top-0 right-2 hover:font-bold"
                on:click={() => (active = false)}
            >
                x
            </button>

            <slot />
        </div>
    </div>
{/if}
