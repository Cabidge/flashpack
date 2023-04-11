<script lang="ts">
    import { invoke } from '$lib/commands';
    import type { Prompt } from '@bindings/Prompt';
    import { onMount } from 'svelte';
    import type { PageData } from './$types';
    import Transition from '$lib/Transition.svelte';
    import PromptView from '$lib/PromptView.svelte';
    import ShowTags from '$lib/ShowTags.svelte';
    import PracticeButton from './PracticeButton.svelte';

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
            script: card.script,
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
            <PracticeButton icon="rotate-left" label="Refresh" on:click={advance} />
        {:else if showAnswer}
            <PracticeButton icon="check-double" label="Easy" on:click={advance} />
            <PracticeButton icon="check" label="Correct" on:click={advance} />
            <PracticeButton icon="xmark" label="Wrong" on:click={advance} />
        {:else}
            <PracticeButton icon="eye" label="Show Answer" on:click={() => (showAnswer = true)} />
        {/if}
    </div>
</div>
