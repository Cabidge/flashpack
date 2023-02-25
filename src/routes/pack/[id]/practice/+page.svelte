<script lang="ts">
    import CardFlip from '@lib/CardFlip.svelte';
    import { onMount } from 'svelte';
    import type { PageData } from './$types';
    import { Deck } from './deck';

    export let data: PageData;

    $: cards = data.cards;
    $: deck = new Deck(cards.length);

    let index: number;

    const advanceCard = () => {
        index = deck.next(index);
    };

    $: card = cards[index];

    onMount(() => {
        if (cards.length < 2) {
            history.back();
        }

        advanceCard();
    });
</script>

{#key { index }}
    <CardFlip {...card} />
{/key}

<br />

<button on:click={advanceCard}>Next Card</button>

<button on:click={() => history.back()}>Stop</button>
