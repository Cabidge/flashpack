<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import type { ModifyCard } from '@bindings/ModifyCard';
    import type { PageData } from './$types';
    import RenameInput from './RenameInput.svelte';
    import ScriptInput from './ScriptInput.svelte';

    export let data: PageData;

    $: ({ id, card } = data);

    $: tags = card.tags;

    let front: string | null, back: string | null;

    $: shouldRename = front !== null || back !== null;

    let additions: Set<string> = new Set();
    let removals: Set<string> = new Set();

    const getChanges = () => {
        let changes: ModifyCard[] = [];

        for (const tag of additions.values()) {
            changes.push({ AddTag: tag });
        }

        for (const tag of removals.values()) {
            changes.push({ RemoveTag: tag });
        }

        if (shouldRename) {
            changes.push({ Rename: { front, back } });
        }

        return changes;
    };

    let tagInput = '';

    $: canSave = additions.size > 0 || removals.size > 0 || shouldRename;

    const addTag = (tag: string) => {
        if (tag === '') {
            return;
        }

        if (tags.includes(tag)) {
            return;
        }

        tags = [...tags, tag].sort();
        additions = additions.add(tag);
    };

    const toggleTag = (tag: string) => {
        removals.has(tag) ? removals.delete(tag) : removals.add(tag);
        removals = removals;
    };

    const submitTag = () => {
        addTag(tagInput);
        tagInput = '';
    };

    const saveChanges = async () => {
        const changes = getChanges();

        if (changes.length === 0) {
            return;
        }

        for (const action of changes) {
            await invoke('modify_card', { id, action });
        }

        additions = new Set();
        removals = new Set();

        await invalidateAll();
    };

    let script: string | null = "";
</script>

<div class="flex h-full flex-col gap-2">
    <div class="flex flex-grow flex-col gap-2 overflow-auto p-2">
        <ScriptInput bind:script={script}/>
        <RenameInput placeholder="front" oldValue={card.front} bind:newValue={front} />
        <RenameInput placeholder="back" oldValue={card.back} bind:newValue={back} />

        <form on:submit|preventDefault={submitTag}>
            <input class="rounded-full px-2" placeholder="add a tag..." bind:value={tagInput} />
        </form>

        <ul class="my-2 flex flex-wrap gap-2">
            {#each tags as tag (tag)}
                <li>
                    <button
                        class="rounded-full bg-slate-100 px-2 hover:bg-slate-200 {removals.has(tag)
                            ? 'text-slate-400 line-through'
                            : 'hover:line-through'}"
                        on:click={() => toggleTag(tag)}
                    >
                        {tag}
                    </button>
                </li>
            {/each}
        </ul>
    </div>

    {#if canSave}
        <button
            class="rounded bg-indigo-500 py-2 font-semibold text-white shadow-md"
            on:click={saveChanges}>Save</button
        >
    {:else}
        <a
            class="rounded bg-slate-100 py-2 text-center font-semibold shadow-md"
            href="/card/{id}/preview"
        >
            Preview
        </a>
    {/if}
</div>
