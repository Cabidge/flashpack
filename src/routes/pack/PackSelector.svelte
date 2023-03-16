<script lang="ts">
    import type { PackSummary } from '@bindings/PackSummary';
    import { flip } from 'svelte/animate';
    import { fade, fly } from 'svelte/transition';
    import PackChoice from './PackChoice.svelte';

    export let packs: PackSummary[];
</script>

{#if packs.length == 0}
    <p in:fade={{ duration: 150, delay: 150 }}>No packs found...</p>
{:else}
    <ul class="flex flex-col">
        {#each packs as pack (pack.id)}
            <li
                in:fly={{ x: -10, duration: 150 }}
                out:fly={{ x: -10, duration: 150 }}
                animate:flip={{ duration: 200 }}
                class="flex"
            >
                <PackChoice {pack} />
            </li>
        {/each}
    </ul>
{/if}
