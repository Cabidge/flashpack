<script lang="ts">
    import { invoke } from '$lib/commands.js';
    import { AppBar } from '@skeletonlabs/skeleton';
    import { onMount } from 'svelte';

    export let data;

    let script = '';
    let template = '';

    onMount(() => {
        script = data.card.script;
        template = data.card.template;
    });

    $: scriptChanged = script !== data.card.script;
    $: templateChanged = template !== data.card.template;
    $: canSave = scriptChanged || templateChanged;

    const saveChanges = async () => {
        await invoke('card_modify', {
            id: data.card.id,
            action: {
                Edit: {
                    label: null,
                    script: scriptChanged ? script : null,
                    template: templateChanged ? template : null
                }
            }
        });
    };
</script>

<AppBar>
    <svelte:fragment slot="lead">
        <a href="/pack/{data.card.pack_id}">
            <i class="fa-solid fa-arrow-left fa-lg" />
        </a>
    </svelte:fragment>

    <h1 class="text-xl font-semibold">
        {data.card.label}
    </h1>

    <svelte:fragment slot="trail">
        {#if canSave}
            <button class="chip variant-filled" on:click={saveChanges}>Save</button>
        {/if}
    </svelte:fragment>
</AppBar>

<div class="p-4">
    <label class="label">
        <span>Script</span>
        <textarea class="textarea font-mono" rows={8} bind:value={script} />
    </label>

    <label class="label">
        <span>Template</span>
        <textarea class="textarea font-mono" rows={8} bind:value={template} />
    </label>
</div>
