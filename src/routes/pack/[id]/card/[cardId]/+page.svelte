<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import type { ModifyCard } from '@bindings/ModifyCard';
    import type { PageData } from './$types';
    import RenameInput from './RenameInput.svelte';

    export let data: PageData;

    $: ({ id, card } = data);

    $: ({ front, back, tags } = card);

    let frontInput: string | null, backInput: string | null;

    let tagModifications: ModifyCard[] = [];
    let modifications: ModifyCard[];

    $: {
        let fullModifications = tagModifications;

        if (frontInput !== null || backInput !== null) {
            fullModifications = [...fullModifications, { Rename: { front: frontInput, back: backInput } }]
        }

        modifications = fullModifications;
    }

    let tagInput = '';

    $: canSave = modifications.length > 0;

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

        for (const action of modifications) {
            await invoke('modify_card', { id, action });
        }

        tagModifications = [];

        await invalidateAll();
    };
</script>

<RenameInput placeholder="front" oldValue={front} bind:newValue={frontInput} />
<RenameInput placeholder="back" oldValue={back} bind:newValue={backInput} />

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
