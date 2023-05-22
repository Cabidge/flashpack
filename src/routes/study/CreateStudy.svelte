<script lang="ts">
    import TagInput from '$lib/TagInput.svelte';
    import { packs } from '$lib/stores/packs';

    let packId: number | null;
</script>

<select bind:value={packId}>
    <option value={null}>All Packs</option>
    {#each $packs as pack (pack.id)}
        <option value={pack.id}>
            {pack.title}
        </option>
    {/each}
</select>

<TagInput />
<TagInput />
