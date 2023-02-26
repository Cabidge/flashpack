<script lang="ts">
    import { page } from '$app/stores';
    import { loadPack, type CardPack } from '@lib/pack';
    import { fly } from 'svelte/transition';

    let cardPack: CardPack | undefined;

    $: {
        (async () => {
            cardPack = await loadPack($page.params.id);
        })();
    }

    const transitionDuration = 100;
</script>

{#if cardPack !== undefined}
    <h1>{cardPack.pack.title} <a href="{$page.url.pathname}/edit">edit</a></h1>

    <h2>Cards ({cardPack.cards.length}) <a href="{$page.url.pathname}/add">add</a></h2>

    {#if cardPack.cards.length > 1}
        <a href="{$page.url.pathname}/practice">begin practice</a>
    {:else}
        <p>Add more cards to begin practicing</p>
    {/if}
{/if}
