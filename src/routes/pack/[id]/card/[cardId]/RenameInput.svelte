<script lang="ts">
    export let oldValue: string;
    export let newValue: string | null;

    $: inputValue = oldValue;

    $: newValue = (oldValue !== inputValue && inputValue !== "") ? inputValue : null;

    export let placeholder = "";
</script>

<input {placeholder} bind:value={inputValue} />