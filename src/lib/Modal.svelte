<script>
    import { fade, fly } from 'svelte/transition';
    import { expoOut } from 'svelte/easing';

    export let active = false;
</script>

{#if active}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div
        on:click|preventDefault={() => (active = false)}
        transition:fade={{ duration: 100 }}
        class="fixed left-0 top-0 z-10 flex h-screen w-screen items-center justify-center overflow-auto bg-black bg-opacity-50"
    >
        <div
            on:click|stopPropagation|preventDefault
            transition:fly={{ y: 50, duration: 380, easing: expoOut }}
            class="relative rounded bg-white p-6 shadow-lg"
        >
            <button
                class="absolute top-0 right-2 hover:font-bold"
                on:click|preventDefault={() => (active = false)}
            >
                x
            </button>

            <slot />
        </div>
    </div>
{/if}
