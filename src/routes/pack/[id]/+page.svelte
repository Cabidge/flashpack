<script lang="ts">
    import type { PageData } from './$types';
    import { page } from '$app/stores';

    export let data: PageData;

    $: pack = data.pack;
    $: cards = data.cards;
</script>

<h1>{pack.title} <a href="{$page.url.pathname}/edit">edit</a></h1>

<h2>Cards ({cards.length}) <a href="{$page.url.pathname}/add">add</a></h2>

{#if cards.length > 1}
    <a href="{$page.url.pathname}/practice">begin practice</a>
{:else}
    <p>Add more cards to begin practicing</p>
{/if}
