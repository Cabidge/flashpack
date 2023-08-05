<script lang="ts">
    // TODO: try out prism.js instead of highlight.js -- their jinja syntax highlighting seems nicer
    import hljs from 'highlight.js';
    import autosize from 'svelte-autosize';

    export let language: string | undefined = undefined;
    export let textarea: HTMLTextAreaElement;
    export let value = '';

    $: highlightResult =
        language === undefined ? hljs.highlightAuto(value) : hljs.highlight(value, { language });
</script>

<!--TODO: handle overflowed text for text areas-->
<div class="relative">
    <textarea
        class="textarea variant-form-material font-mono text-transparent caret-white backdrop-blur-0"
        rows={1}
        bind:this={textarea}
        bind:value
        use:autosize
    />
    <pre class="pointer-events-none absolute top-2 left-3">{@html highlightResult.value}</pre>
</div>
