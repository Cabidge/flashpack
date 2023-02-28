<script lang="ts">
    import { page } from '$app/stores';
    import ModalContext from '@lib/ModalContext.svelte';
    import RenamePack from '@lib/RenamePack.svelte';
    import type { PageData } from './$types';

    export let data: PageData;

    $: pack = data.pack;
    $: cards = data.cards;
</script>

<ModalContext let:open let:close>
    <h1>{pack.title} <button on:click={open}>rename</button></h1>

    <RenamePack slot="modal" on:close={close} {pack} />
</ModalContext>

<h2>Cards ({cards.length}) <a href="{$page.url.pathname}/add">add</a></h2>

{#if cards.length > 1}
    <a href="{$page.url.pathname}/practice">begin practice</a>
{:else}
    <p>Add more cards to begin practicing</p>
{/if}
