<script lang="ts">
    import TagInput from '$lib/TagInput.svelte';
    import { getModalContext } from '$lib/modals';

    export let packId: number;

    const { close } = getModalContext();
</script>

<form action="/study/practice" class="flex flex-col gap-2" on:submit={close}>
    <input type="hidden" name="pack" value={packId} />

    <span>Included Tags</span>
    <TagInput name="include[]" />

    <span>Excluded Tags</span>
    <TagInput name="exclude[]" />

    <span>Max Question Count (0 for unlimited)</span>
    <input
        type="number"
        name="limit"
        id="limit"
        min="0"
        value="0"
        class="rounded border border-slate-300 p-1"
    />

    <button class="mt-2 p-2 shadow rounded bg-violet-500 hover:bg-violet-600 font-semibold text-white" type="submit">Start Practice</button>
</form>
