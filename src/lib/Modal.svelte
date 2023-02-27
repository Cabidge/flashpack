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
            class="bg-white shadow-lg p-4 rounded"
        >
            <slot />
        </div>
    </div>
{/if}
