<script lang="ts">
    export let oldValue: string;
    export let newValue: string | null;

    $: inputValue = oldValue;

    $: newValue = oldValue !== inputValue && inputValue !== '' ? inputValue : null;

    export let placeholder = '';
</script>

<textarea
    class="rounded border border-slate-300 p-1 font-mono shadow"
    draggable="false"
    rows="5"
    {placeholder}
    bind:value={inputValue}
/>
