<script lang="ts">
    import { createEventDispatcher } from "svelte";


    export let label: string;
    export let tags: string[] = [];

    const dispatch = createEventDispatcher<{ edit: undefined }>();
</script>

<button
    class="relative group flex h-32 flex-col items-center gap-2 rounded border-2 border-transparent bg-white p-3 text-lg shadow transition hover:-translate-y-1 hover:border-indigo-400 hover:shadow-md"
    on:click
>
    <span class="flex flex-grow items-center">
        {label}
    </span>

    <ul class="flex flex-shrink flex-row flex-wrap justify-center gap-2">
        {#each tags as tag}
            <li class="rounded-full bg-slate-100 px-2 text-sm hover:bg-slate-200">{tag}</li>
        {/each}
    </ul>

    <button on:click={() => dispatch("edit")} class="absolute hidden group-hover:inline rounded text-white top-1 right-1 aspect-square hover:bg-indigo-500 bg-slate-800">EDIT</button>
</button>
