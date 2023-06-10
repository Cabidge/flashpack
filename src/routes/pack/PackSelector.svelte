<script lang="ts">
    import type { Pack } from '@bindings/Pack';
    import PackChoice from './PackChoice.svelte';

    export let packs: Pack[];
    export let activePack: number | null;
</script>

{#if packs.length == 0}
    <p>No packs found...</p>
{:else}
    <ul class="flex flex-col">
        {#each packs as pack (pack.id)}
            <li class="flex">
                <PackChoice {pack} bind:activePack />
            </li>
        {/each}
    </ul>
{/if}
