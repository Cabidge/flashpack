<script lang="ts">
    import type { Prompt } from '@bindings/Prompt';

    export let prompt: Prompt;
    export let tags: string[] = [];
    export let showAnswer = true;
</script>

<div class="prose">
    {@html prompt.question}
</div>

{#if tags.length > 0}
    <ul class="flex flex-row gap-2 pt-2">
        {#each tags as tag (tag)}
            <li class="rounded-full bg-slate-100 px-2 text-sm">{tag}</li>
        {/each}
    </ul>
{/if}

{#if showAnswer}
    <hr class="my-2" />

    <div class="prose">
        {@html prompt.answer}
    </div>
{/if}
