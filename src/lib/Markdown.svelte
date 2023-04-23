<script lang="ts">
    import { invoke } from './commands';

    export let content: string;

    $: rendered = invoke('render_markdown', { markdown: content });
</script>

{#await rendered then html}
    {@html html}
{:catch err}
    <p>Markdown error: {err}</p>
{/await}
