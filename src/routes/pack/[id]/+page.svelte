<script lang="ts">
    import { page } from '$app/stores';
    import { loadPack } from '@lib/pack';

    let cardPack = loadPack($page.params.id);
</script>

{#await cardPack}
    <p>Loading...</p>
{:then cardPack}
    <h1>{cardPack.pack.title} <a href="{$page.url.pathname}/edit">edit</a></h1>

    <h2>Cards ({cardPack.cards.length}) <a href="{$page.url.pathname}/add">add</a></h2>

    {#if cardPack.cards.length > 1}
        <a href="{$page.url.pathname}/practice">begin practice</a>
    {:else}
        <p>Add more cards to begin practicing</p>
    {/if}
{/await}
