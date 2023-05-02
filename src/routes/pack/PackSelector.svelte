<script lang="ts">
    import type { PackWithId } from '$lib/stores/packs';
    import PackChoice from './PackChoice.svelte';

    export let packs: PackWithId[];
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
