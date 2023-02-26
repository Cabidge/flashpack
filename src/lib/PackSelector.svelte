<script lang="ts">
    import { flip } from 'svelte/animate';
    import { fade } from 'svelte/transition';
    import type { Packs } from './packs';
    import SmartLink from './SmartLink.svelte';

    export let packs: Packs;
</script>

<ul>
    {#each packs.list as pack (pack.id)}
        <li transition:fade={{ duration: 150 }} animate:flip={{ duration: 200 }}>
            <SmartLink href="/pack/{pack.id}" disabled={packs.loading}>{pack.title}</SmartLink>
        </li>
    {/each}
</ul>
