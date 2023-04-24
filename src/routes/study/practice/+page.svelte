<script lang="ts">
    import type { PageData } from './$types';
    import Transition from '$lib/Transition.svelte';
    import PromptView from '$lib/PromptView.svelte';
    import ShowTags from '$lib/ShowTags.svelte';
    import PracticeButton from './PracticeButton.svelte';
    import type { FullPrompt } from './+page';

    export let data: PageData;

    $: questions = data.questions;
    let index = 0;

    let prompt: FullPrompt | undefined;
    $: prompt = questions[index];

    let showAnswer = false;

    const advance = async () => {
        index += 1;
        showAnswer = false;
    };
</script>

<div class="flex h-full flex-col">
    <Transition class="flex-grow overflow-auto px-6" key={prompt}>
        {#if questions.length === 0}
            <p>No questions generated...</p>
        {:else if prompt === undefined}
            <p>End of questions...</p>
        {:else}
            <p>{index + 1}/{questions.length}</p>

            {#if prompt.tags.length > 0}
                <ShowTags tags={prompt.tags} />
            {/if}

            <PromptView {prompt} {showAnswer} />
        {/if}
    </Transition>

    {#if prompt !== undefined}
        <div class="flex w-full items-center justify-center gap-2 bg-slate-100 py-6">
            {#if showAnswer}
                <PracticeButton icon="check" label="Correct" on:click={advance} />
                <PracticeButton icon="xmark" label="Wrong" on:click={advance} />
            {:else}
                <PracticeButton icon="eye" label="Show Answer" on:click={() => (showAnswer = true)} />
            {/if}
        </div>
    {/if}
</div>
