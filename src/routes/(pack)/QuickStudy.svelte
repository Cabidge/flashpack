<script lang="ts">
    import TagInput from '$lib/TagInput.svelte';
    import { getModalContext } from '$lib/modals';

    export let packId: number;

    const { close } = getModalContext();
</script>

<form action="/study/practice" class="flex max-w-xs flex-col gap-2" on:submit={close}>
    <input type="hidden" name="pack" value={packId} />

    <label for="include">
        <span>Included Tags</span>
        <TagInput class="w-full" id="include" name="include[]" />
    </label>

    <label for="exclude">
        <span>Excluded Tags</span>
        <TagInput class="w-full" id="exclude" name="exclude[]" />
    </label>

    <label for="limit">
        <span>Max Question Count (0 for unlimited)</span>
        <input
            type="number"
            name="limit"
            id="limit"
            min="0"
            value="0"
            class="w-full rounded border border-slate-300 p-1"
        />
    </label>

    <button
        class="mt-2 rounded bg-violet-500 p-2 font-semibold text-white shadow hover:bg-violet-600"
        type="submit"
    >
        Start Practice
    </button>
</form>
