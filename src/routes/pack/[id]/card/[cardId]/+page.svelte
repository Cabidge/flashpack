<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import type { ModifyCard } from '@bindings/ModifyCard';
    import type { PageData } from './$types';

    export let data: PageData;

    $: id = data.id;
    $: card = data.card;

    $: front = card.front;
    $: back = card.back;
    $: tags = card.tags;

    $: frontInput = front;
    $: backInput = back;

    let tagModifications: ModifyCard[] = [];

    let modifications: ModifyCard[];

    $: {
        let fullModifications = tagModifications;

        if (front !== frontInput) {
            // TODO push rename front
        }

        if (back !== backInput) {
            // TODO push rename back
        }

        modifications = fullModifications;
    }

    let tagInput = '';

    $: canSave = frontInput !== '' && backInput !== '' && modifications.length > 0;

    const addTag = () => {
        if (tagInput === '') {
            return;
        }

        const tag = tagInput;

        tagInput = '';

        if (tags.includes(tag)) {
            return;
        }

        tags = [...tags, tag].sort();
        tagModifications = [...tagModifications, { AddTag: tag }];
    };

    const saveChanges = async () => {
        if (!canSave) {
            return;
        }

        for (const action of tagModifications) {
            await invoke('modify_card', { id, action });
        }

        tagModifications = [];

        await invalidateAll();
    };
</script>

<input placeholder="front" bind:value={frontInput} />
<input placeholder="back" bind:value={backInput} />

<form on:submit|preventDefault={addTag}>
    <input placeholder="add a tag..." bind:value={tagInput} />
</form>

<ul>
    {#each tags as tag (tag)}
        <li>
            {tag}
        </li>
    {/each}
</ul>

{#if canSave}
    <button on:click={saveChanges}>Save</button>
{/if}
