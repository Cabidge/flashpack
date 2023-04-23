<script lang="ts">
    import type { Prompt } from '@bindings/Prompt';
    import Markdown from './Markdown.svelte';

    export let prompt: Prompt;
    export let showAnswer = true;
</script>

<div class="prose">
    <Markdown content={prompt.front} />

    {#if showAnswer}
        <hr class="my-2" />

        <Markdown content={prompt.back} />
    {/if}
</div>
