<script lang="ts">
    export let name: string;

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
    bind:value={inputValue}
    on:keypress={(e) => {
        if (e.key === 'Enter') {
            e.preventDefault();
            handleEnter();
        }
    }}
/>

{#each tags as tag (tag)}
    <div>
        <button type="button" on:click={() => removeTag(tag)}> x </button>
        {tag}
    </div>
    <input type="hidden" {name} value={tag} />
{/each}
