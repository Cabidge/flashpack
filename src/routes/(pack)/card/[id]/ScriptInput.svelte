<script lang="ts">
    import CodeMirror from 'svelte-codemirror-editor';
    import { rust } from '@codemirror/lang-rust';

    export let script: string | null;
    let enabled = false;

    $: script = enabled ? script ?? '' : null;
</script>

<input type="checkbox" bind:checked={enabled} />

{#if script !== null}
    <CodeMirror bind:value={script} lang={rust()} />
{/if}
