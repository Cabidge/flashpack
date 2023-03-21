<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import type { PageData } from './$types';

    export let data: PageData;

    $: id = data.id;
    $: card = data.card;

    let tagInput = '';

    const addTag = async () => {
        if (tagInput === '') {
            return;
        }

        await invoke('modify_card', { id, action: { AddTag: tagInput } });
        await invalidateAll();

        tagInput = '';
    };
</script>

<form on:submit|preventDefault={addTag}>
    <input placeholder="add a tag..." bind:value={tagInput} />
</form>

<ul>
    {#each card.tags as tag (tag)}
        <li>
            {tag}
        </li>
    {/each}
</ul>
