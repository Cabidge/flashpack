<script lang="ts">
    import type { PageData } from './$types';
    import RenamePack from '@lib/RenamePack.svelte';

    export let data: PageData;

    type Editing = { target: 'title' } | { target: 'card'; id: string } | null;

    let editing: Editing = null;

    $: notEditing = editing === null;
    $: isEditingTitle = editing?.target === 'title';
    $: isEditingCard = editing?.target === 'card';
</script>

<h2>{data.pack.title}</h2>

<button on:click={() => (editing = { target: 'title' })} disabled={!notEditing}>rename</button>

{#if isEditingTitle}
    <RenamePack
        bind:title={data.pack.title}
        bind:id={data.pack.id}
        on:end={() => (editing = null)}
    />
{/if}
