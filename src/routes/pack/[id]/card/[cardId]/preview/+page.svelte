<script lang="ts">
    import { page } from '$app/stores';
    import PromptView from '$lib/PromptView.svelte';
    import { invoke } from '$lib/commands';
    import { cardsContext } from '$lib/stores/cards';
    import type { Prompt } from '@bindings/Prompt';
    import { derived } from 'svelte/store';

    const cards = cardsContext.get();

    const packId = derived(page, ($page) => parseInt($page.params.id));
    const id = derived(page, ($page) => parseInt($page.params.cardId));
    const card = cards.get(id);

    let showAnswer = false;

    $: packHref = `/pack/${$packId}`;
    $: cardHref = `${packHref}/card/${$id}`;

    let prompt: Prompt;

    $: {
        (async () => {
            const { script, front, back } = $card;

            prompt = script === null
                ? { front, back }
                : await invoke('generate_prompt', { script, front, back });
        })()
    }
</script>

<a href={packHref}>Return to Pack</a>
<a href={cardHref}>Edit</a>

<label for="showAnswer">
    <input type="checkbox" name="showAnswer" id="showAnswer" bind:checked={showAnswer} />
    Show Answer
</label>

<PromptView {prompt} {showAnswer} />
