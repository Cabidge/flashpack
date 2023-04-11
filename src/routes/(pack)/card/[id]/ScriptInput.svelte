<script lang="ts">
    import CodeMirror from 'svelte-codemirror-editor';
    import { rust } from '@codemirror/lang-rust';

    export let script: string | null;
</script>

<input
    type="checkbox"
    checked={script !== null}
    on:click={(e) => {
        if (e.currentTarget.checked) {
            script ??= '';
        } else {
            script = null;
        }
    }}
/>

{#if script !== null}
    <CodeMirror bind:value={script} lang={rust()} />
{/if}
