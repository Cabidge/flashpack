<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
    import { loadPack, type CardPack } from '@lib/pack';

    let cardPack: CardPack | undefined;

    $: {
        (async () => {
            try {
                cardPack = await loadPack($page.params.id);
            } catch {
                await goto('/pack');
            }
        })();
    }
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
