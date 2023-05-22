<script lang="ts">
    export let name: string | undefined = undefined;
    export let id: string | undefined = undefined;

    let clazz = '';
    export { clazz as class };

    let inputValue: string = '';

    $: tagInput = inputValue.trim().toLowerCase();

    let tags: string[] = [];

    const handleEnter = () => {
        if (tagInput === '') {
            return;
        }

        addTag(tagInput);

        inputValue = '';
    };

    const addTag = (tag: string) => {
        if (tags.includes(tag)) {
            return;
        }

        tags = [...tags, tag];
    };

    const removeTag = (tag: string) => {
        tags = tags.filter((t) => t !== tag);
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
    {#each tags as tag (tag)}
        <li class="rounded-full bg-slate-200 px-2">
            <button type="button" on:click={() => removeTag(tag)}> x </button>
            {tag}
        </li>
    {/each}
</ul>

{#each tags as tag}
    <input type="hidden" {name} value={tag} />
{/each}
