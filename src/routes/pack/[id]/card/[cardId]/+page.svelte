<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import type { ModifyCard } from '@bindings/ModifyCard';
    import type { PageData } from './$types';

    export let data: PageData;

    $: ({ id, card } = data);

    $: ({ front, back, tags } = card);

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

    const addTag = (tag: string) => {
        if (tag === '') {
            return;
        }

        if (tags.includes(tag)) {
            return;
        }

        tags = [...tags, tag].sort();
        tagModifications = [...tagModifications, { AddTag: tag }];
    };

    const submitTag = () => {
        addTag(tagInput);
        tagInput = "";
    }

    const removeTag = (tag: string) => {
        tags = tags.filter((t) => t !== tag);
        tagModifications = [...tagModifications, { RemoveTag: tag }];
    }

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

<form on:submit|preventDefault={submitTag}>
    <input placeholder="add a tag..." bind:value={tagInput} />
</form>

<ul>
    {#each tags as tag (tag)}
        <li>
            <button class="hover:line-through" on:click={() => removeTag(tag)}>
                {tag}
            </button>
        </li>
    {/each}
</ul>

{#if canSave}
    <button on:click={saveChanges}>Save</button>
{/if}
