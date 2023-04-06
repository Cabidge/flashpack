<script lang="ts">
    import { invoke } from '$lib/commands';
    import type { Prompt } from '@bindings/Prompt';
    import { onMount } from 'svelte';
    import type { PageData } from './$types';
    import Transition from '$lib/Transition.svelte';
    import PromptView from '$lib/PromptView.svelte';
    import ShowTags from '$lib/ShowTags.svelte';

    export let data: PageData;

    type FullPrompt = {
        prompt: Prompt;
        tags: string[];
    };

    let prompt: FullPrompt | null = null;

    let showAnswer = false;

    $: if (prompt) {
        showAnswer = false;
    }

    const advance = async () => {
        prompt = await nextPrompt();
    };

    const nextPrompt = async () => {
        const cardId = await invoke('deal_card', { dealerId: data.id });

        if (cardId === null) {
            return null;
        }

        const card = await invoke('get_card', { id: cardId });

        const partialPrompt = await invoke('generate_prompt', {
            script: null,
            question: card.front,
            answer: card.back
        });

        return {
            prompt: partialPrompt,
            tags: card.tags
        };
    };

    onMount(advance);
</script>

<div class="flex h-full flex-col">
    <h1 class="mx-6 my-4 text-lg font-bold">{data.dealer.title} practice</h1>

    <Transition class="flex-grow overflow-auto px-6" key={prompt}>
        {#if prompt === null}
            <p>Unable to generate prompt...</p>
        {:else}
            {#if prompt.tags.length > 0}
                <ShowTags tags={prompt.tags} />
            {/if}

            <PromptView prompt={prompt.prompt} {showAnswer} />
        {/if}
    </Transition>

    <div class="flex w-full items-center justify-center gap-2 bg-slate-100 py-6">
        {#if prompt === null}
            <button class="rounded bg-white py-1 px-2 shadow" on:click={advance}>
                <i class="fa-solid fa-rotate-left" /> Refresh
            </button>
        {:else if showAnswer}
            <button class="rounded bg-white py-1 px-2 shadow" on:click={advance}>
                <i class="fa-solid fa-check-double" /> Easy
            </button>
            <button class="rounded bg-white py-1 px-2 shadow" on:click={advance}>
                <i class="fa-solid fa-check" /> Correct
            </button>
            <button class="rounded bg-white py-1 px-2 shadow" on:click={advance}>
                <i class="fa-solid fa-xmark" /> Wrong
            </button>
        {:else}
            <button class="rounded bg-white py-1 px-2 shadow" on:click={() => (showAnswer = true)}>
                <i class="fa-solid fa-eye" /> Show Answer
            </button>
        {/if}
    </div>
</div>
