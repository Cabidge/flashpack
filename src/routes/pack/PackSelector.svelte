<script lang="ts">
    import type { Pack } from '@bindings/Pack';
    import { flip } from 'svelte/animate';
    import { fade } from 'svelte/transition';
    import PackSelectable from './PackSelectable.svelte';

    export let packs: Pack[];
</script>

<ul>
    {#each packs as pack (pack.id)}
        <li transition:fade={{ duration: 150 }} animate:flip={{ duration: 200 }}>
            <PackSelectable bind:pack />
        </li>
    {/each}
</ul>
