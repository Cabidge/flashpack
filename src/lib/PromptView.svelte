<script lang="ts">
    import type { Prompt } from '@bindings/Prompt';

    export let prompt: Prompt;
    export let showAnswer = true;
</script>

<div class="prose">
    {@html prompt.question}
</div>

{#if showAnswer}
    <hr class="my-2" />

    <div class="prose">
        {@html prompt.answer}
    </div>
{/if}
