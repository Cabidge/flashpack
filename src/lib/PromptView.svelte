<script lang="ts">
    import type { Prompt } from "@bindings/Prompt";

    export let prompt: Prompt;
    export let showAnswer = true;
</script>

<div>
    {prompt.question}
</div>

{#if prompt.tags.length > 0}
    <ul class="flex flex-row gap-2 pt-2">
        {#each prompt.tags as tag (tag)}
            <li class="rounded-full bg-slate-100 px-2 text-sm">{tag}</li>
        {/each}
    </ul>
{/if}

{#if showAnswer}
    <hr class="my-2" />

    <div>
        {prompt.answer}
    </div>
{/if}