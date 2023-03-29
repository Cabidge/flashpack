<script lang="ts">
    import { invoke } from '$lib/commands';
    import type { Prompt } from '@bindings/Prompt';
    import { onMount } from 'svelte';
    import type { PageData } from './$types';
    import Transition from '$lib/Transition.svelte';
    import PromptView from '$lib/PromptView.svelte';

    export let data: PageData;

    let prompt: Prompt | null = null;

    let showAnswer = false;

    $: if (prompt) {
        showAnswer = false;
    }

    const advanceQuestion = async () => {
        const cardId = await invoke('deal_card', { dealerId: data.id });
        prompt = cardId === null ? null : await invoke('generate_prompt', { cardId });
    };

    onMount(advanceQuestion);
</script>

<div class="flex h-full flex-col">
    <h1 class="mx-6 my-4 text-lg font-bold">{data.dealer.title} practice</h1>

    <Transition class="flex-grow overflow-auto px-6" key={prompt}>
        {#if prompt === null}
            <p>Unable to generate prompt...</p>
        {:else}
            <PromptView {prompt} {showAnswer} />
        {/if}
    </Transition>

    <div class="flex w-full items-center justify-center gap-4 bg-slate-100 py-6">
        {#if prompt === undefined}
            <button class="rounded bg-white py-1 px-2 shadow" on:click={advanceQuestion}>
                Retry
            </button>
        {:else if showAnswer}
            <button class="rounded bg-white py-1 px-2 shadow" on:click={advanceQuestion}>
                Correct
            </button>
            <button class="rounded bg-white py-1 px-2 shadow" on:click={() => (showAnswer = false)}>
                Hide Answer
            </button>
            <button class="rounded bg-white py-1 px-2 shadow" on:click={advanceQuestion}>
                Wrong
            </button>
        {:else}
            <button class="rounded bg-white py-1 px-2 shadow" on:click={() => (showAnswer = true)}>
                Show Answer
            </button>
        {/if}
    </div>
</div>
