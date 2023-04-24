<script lang="ts">
    import PromptView from '$lib/PromptView.svelte';
    import type { PageData } from './$types';

    export let data: PageData;

    $: ({ id, card, prompt } = data);

    let showAnswer = false;
</script>

<a href="/pack/{card.pack_id}">Return to Pack</a>
<a href="/card/{id}">Edit</a>

<label for="showAnswer">
    <input type="checkbox" name="showAnswer" id="showAnswer" bind:checked={showAnswer} />
    Show Answer
</label>

<PromptView {prompt} {showAnswer} />
