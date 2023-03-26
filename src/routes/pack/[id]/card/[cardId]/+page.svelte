<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import type { ModifyCard } from '@bindings/ModifyCard';
    import type { PageData } from './$types';
    import RenameInput from './RenameInput.svelte';

    export let data: PageData;

    $: ({ id, card } = data);

    $: tags = card.tags.map((tag) => ({ tag, removed: false }));

    let front: string | null, back: string | null;

    let tagModifications: ModifyCard[] = [];
    let modifications: ModifyCard[];

    $: {
        modifications = tagModifications;

        if (front!== null || back !== null) {
            modifications = [...modifications, { Rename: { front, back } }]
        }
    }

    let tagInput = '';

    $: canSave = modifications.length > 0;

    const addTag = (tag: string) => {
        if (tag === '') {
            return;
        }

        if (tags.map(({ tag }) => tag).includes(tag)) {
            return;
        }

        tags = [...tags, { tag, removed: false }].sort(({ tag: tagA }, { tag: tagB }) => tagA.localeCompare(tagB));
        tagModifications = [...tagModifications, { AddTag: tag }];
    };

    const submitTag = () => {
        addTag(tagInput);
        tagInput = "";
    }

    const removeTag = (tag: string) => {
        let index = tags.findIndex(({ tag: t }) => t == tag);

        if (index === -1 || tags[index].removed) {
            return;
        }

        tags[index].removed = true;
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

<RenameInput placeholder="front" oldValue={card.front} bind:newValue={front} />
<RenameInput placeholder="back" oldValue={card.back} bind:newValue={back} />

<form on:submit|preventDefault={submitTag}>
    <input placeholder="add a tag..." bind:value={tagInput} />
</form>

<ul>
    {#each tags as { tag, removed } (tag)}
        <li>
            <button class="hover:line-through" class:line-through={removed} disabled={removed} on:click={() => removeTag(tag)}>
                {tag}
            </button>
        </li>
    {/each}
</ul>

{#if canSave}
    <button on:click={saveChanges}>Save</button>
{/if}
