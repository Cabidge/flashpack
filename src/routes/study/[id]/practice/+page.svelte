<script lang="ts">
    import { invoke } from '$lib/commands';
    import type { Prompt } from '@bindings/Prompt';
    import { onMount } from 'svelte';
    import type { PageData } from './$types';

    export let data: PageData;

    let prompt: Prompt | undefined;

    let showAnswer = false;

    $: if (prompt) {
        showAnswer = false;
    }

    const advanceQuestion = async () => {
        const cardId = await invoke('deal_card', { dealerId: data.id });

        if (cardId === undefined) {
            return;
        }

        prompt = await invoke('generate_prompt', { cardId });
    };

    onMount(advanceQuestion);
</script>

<h1>{data.dealer.title} practice</h1>

{#if prompt === undefined}
    <p>Unable to generate prompt...</p>
    <button on:click={advanceQuestion}>try again</button>
{:else}
    <h2>{prompt.question}</h2>
    <p>tags</p>
    <ul>
        {#each prompt.tags as tag (tag)}
            <li>{tag}</li>
        {/each}
    </ul>

    {#if showAnswer}
        <p>{prompt.answer}</p>
        <button on:click={advanceQuestion}>next question</button>
    {:else}
        <button on:click={() => (showAnswer = true)}>show answer</button>
    {/if}
{/if}
