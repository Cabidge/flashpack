<script lang="ts">
    import type { Prompt } from '@bindings/Prompt';
    import PromptHalf from './PromptHalf.svelte';

    export let prompt: Prompt;
    export let showAnswer = true;
</script>

<PromptHalf input={prompt.front} />

{#if showAnswer}
    <hr class="my-2" />

    <PromptHalf input={prompt.back} />
{/if}
