<script lang="ts">
    import TagInput from '$lib/TagInput.svelte';

    export let packId: number;
</script>

<form action="/study/practice">
    <input type="hidden" name="pack" value={packId} />

    <div>Included Tags</div>
    <TagInput name="include[]" />

    <div>Excluded Tags</div>
    <TagInput name="exclude[]" />

    <br />

    <label for="limit">
        Question Count
        <input type="number" name="limit" id="limit" min="0" value="0" />
    </label>

    <button type="submit">Start Practice</button>
</form>
