<script lang="ts">
    import { invalidateAll } from "$app/navigation";
    import { invoke } from "$lib/commands";
    import type { ModifyFilter } from "@bindings/ModifyFilter";
    import type { PageData } from "./$types";

    export let data: PageData;

    $: ({ id, filter } = data);
    $: ({ tags } = filter);

    const modifyFilter = (action: ModifyFilter) => invoke("modify_filter", { id, action });

    const excludeTag = async (tag: string, exclusion: boolean) => {
        await modifyFilter({ SetExclusion: [tag, exclusion] });
        await invalidateAll();
    }

    let tagInput = "";

    const addTag = async () => {
        if (tagInput === "") {
            return;
        }

        await modifyFilter({ AddTag: tagInput });
        tagInput = "";

        await invalidateAll();
    }
</script>

<h1>{filter.label}</h1>

<form on:submit|preventDefault={addTag}>
    <input bind:value={tagInput} placeholder="add tag" required />
</form>

<ul>
    {#each tags as tag (tag.tag)}
        <li>
            <button on:click={() => excludeTag(tag.tag, !tag.exclude)}>{tag.exclude ? "-" : "+"}</button>
            {tag.tag}
        </li>
    {/each}
</ul>
