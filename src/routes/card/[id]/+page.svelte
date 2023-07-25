<script lang="ts">
    import { invoke } from '$lib/commands.js';
    import { Tab, TabGroup } from '@skeletonlabs/skeleton';
    import { onMount } from 'svelte';

    export let data;

    let tabSet = 0;

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

<a href="/pack/{data.card.pack_id}">Return to Pack</a>

<h1>{data.card.label}</h1>

{#if canSave}
    <button class="chip variant-filled" on:click={saveChanges}>Save</button>
{/if}

<TabGroup>
    <Tab class="relative" bind:group={tabSet} name="script" value={0}>
        {#if scriptChanged}
            <span class="badge-icon variant-filled absolute top-0 right-0" />
        {/if}
        <span>Script</span>
    </Tab>
    <Tab class="relative" bind:group={tabSet} name="template" value={1}>
        {#if templateChanged}
            <span class="badge-icon variant-filled absolute top-0 right-0" />
        {/if}
        <span>Template</span>
    </Tab>

    <svelte:fragment slot="panel">
        {#if tabSet === 0}
            <textarea class="textarea" rows={10} bind:value={script} />
        {:else if tabSet === 1}
            <textarea class="textarea" rows={10} bind:value={template} />
        {/if}
    </svelte:fragment>
</TabGroup>
