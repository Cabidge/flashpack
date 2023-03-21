<script lang="ts">
    import type { CardSummary } from '@bindings/CardSummary';

    export let card: CardSummary;

    let showAnswer = false;
</script>

<h1>{card.front}</h1>

<button on:click={() => (showAnswer = !showAnswer)}>toggle answer</button>

{#if showAnswer}
    <div class="border-b-2" />
    <h2>{card.back}</h2>
{/if}
