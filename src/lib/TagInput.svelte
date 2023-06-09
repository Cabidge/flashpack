<script context="module" lang="ts">
    import { derived, writable } from 'svelte/store';

    const formatTag = (tag: string) => tag.trim().toLowerCase();

    export const createTagStore = (initial: string[] = []) => {
        const tags = writable(
            new Set<string>(initial.map(formatTag).filter((tag) => tag.length > 0))
        );

        const add = (tag: string) => {
            tag = formatTag(tag);

            if (tag.length > 0) {
                tags.update(($tags) => $tags.add(tag));
            }
        };

        const remove = (tag: string) => {
            tag = formatTag(tag);

            tags.update(($tags) => {
                $tags.delete(tag);
                return $tags;
            });
        };

        const { subscribe } = derived(tags, ($tags) => [...$tags]);

        return {
            subscribe,
            add,
            remove
        };
    };

    export type TagStore = ReturnType<typeof createTagStore>;
</script>

<script lang="ts">
    export let name: string | undefined = undefined;
    export let id: string | undefined = undefined;

    let clazz = '';
    export { clazz as class };

    let inputValue: string = '';

    export let value: TagStore = createTagStore();

    const handleEnter = () => {
        value.add(inputValue);
        inputValue = '';
    };
</script>

<input
    {id}
    bind:value={inputValue}
    on:keypress={(e) => {
        if (e.key === 'Enter') {
            e.preventDefault();
            handleEnter();
        }
    }}
    type="text"
    class="{clazz} rounded border border-slate-300 p-1"
/>

<ul class="flex flex-wrap gap-1">
    {#each $value as tag (tag)}
        <li class="rounded-full bg-slate-200 px-2">
            <button type="button" on:click={() => value.remove(tag)}> x </button>
            {tag}
        </li>
    {/each}
</ul>

{#each $value as tag}
    <input type="hidden" {name} value={tag} />
{/each}
