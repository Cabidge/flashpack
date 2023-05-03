<script lang="ts">
    import PromptView from "$lib/PromptView.svelte";
    import type { Prompt } from "@bindings/Prompt";
    import PracticeButton from "./PracticeButton.svelte";

    export let questions: Prompt[];
    export let index = 0;

    let prompt: Prompt | undefined;
    $: prompt = questions[index];

    let showAnswer = false;

    const advance = async () => {
        index += 1;
        showAnswer = false;
    };
</script>

<div class="flex h-full flex-col">
    <div class="flex-grow overflow-auto px-6">
        {#if questions.length === 0}
            <p>No questions generated...</p>
        {:else if prompt === undefined}
            <p>End of questions...</p>
        {:else}
            <p>{index + 1}/{questions.length}</p>
            <PromptView {prompt} {showAnswer} />
        {/if}
    </div>

    {#if prompt !== undefined}
        <div class="flex w-full items-center justify-center gap-2 bg-slate-100 py-6">
            {#if showAnswer}
                <PracticeButton icon="check" label="Correct" on:click={advance} />
                <PracticeButton icon="xmark" label="Wrong" on:click={advance} />
            {:else}
                <PracticeButton
                    icon="eye"
                    label="Show Answer"
                    on:click={() => (showAnswer = true)}
                />
            {/if}
        </div>
    {/if}
</div>