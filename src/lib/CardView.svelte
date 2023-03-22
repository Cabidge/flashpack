<script lang="ts">
    import type { Card } from '@bindings/Card';

    export let card: Card;

    let showAnswer = false;
</script>

<h1>{card.front}</h1>

<button on:click={() => (showAnswer = !showAnswer)}>toggle answer</button>

{#if showAnswer}
    <div class="border-b-2" />
    <h2>{card.back}</h2>
{/if}
