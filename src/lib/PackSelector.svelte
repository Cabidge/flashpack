<script lang="ts">
    import type { Pack } from '@bindings/Pack';
    import { flip } from 'svelte/animate';
    import { fade } from 'svelte/transition';
    import SmartLink from './SmartLink.svelte';

    export let packs: Pack[];
    export let disabled = false;
</script>

<ul>
    {#each packs as pack (pack.id)}
        <li transition:fade={{ duration: 150 }} animate:flip={{ duration: 200 }}>
            <SmartLink href="/pack/{pack.id}" {disabled}>{pack.title}</SmartLink>
        </li>
    {/each}
</ul>
