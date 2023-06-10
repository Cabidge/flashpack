<script lang="ts">
    import { page } from '$app/stores';
    import PromptView from '$lib/PromptView.svelte';
    import { invoke } from '$lib/commands';
    import { cardsContext } from '$lib/stores/cards';
    import { derived } from 'svelte/store';

    const cards = cardsContext.get();

    const packId = derived(page, ($page) => parseInt($page.params.id));
    const id = derived(page, ($page) => parseInt($page.params.cardId));
    const card = cards.get(id);

    let showAnswer = false;

    $: packHref = `/pack/${$packId}`;
    $: cardHref = `${packHref}/card/${$id}`;

    $: prompt = invoke('generate_prompt', $card);
</script>

<a href={packHref}>Return to Pack</a>
<a href={cardHref}>Edit</a>

<label>
    <input type="checkbox" name="showAnswer" bind:checked={showAnswer} />
    Show Answer
</label>

{#await prompt}
    <p>Generating Preview...</p>
{:then prompt}
    <PromptView {prompt} {showAnswer} />
{:catch}
    <p>Error Generating Preview</p>
{/await}
