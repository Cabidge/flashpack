<script lang="ts">
    import { page } from '$app/stores';
    import type { Pack } from '@bindings/Pack';
    import { flip } from 'svelte/animate';
    import { fade } from 'svelte/transition';
    import PackSelectable from './PackSelectable.svelte';

    export let packs: Pack[];

    let selectedId: undefined | string;

    $: {
        let path = $page.url.pathname.split('/');

        if (path[1].toLowerCase() == 'pack') {
            selectedId = path[2];
        } else {
            selectedId = undefined;
        }
    }
</script>

<ul>
    {#each packs as pack (pack.id)}
        <li transition:fade={{ duration: 150 }} animate:flip={{ duration: 200 }}>
            <PackSelectable bind:pack bind:selectedId />
        </li>
    {/each}
</ul>
