<script lang="ts">
    import { goto } from '$app/navigation';
    import { invoke } from '$lib/commands.js';
    import { AppBar } from '@skeletonlabs/skeleton';
    import { onMount, tick } from 'svelte';
    import autosize from 'svelte-autosize';

    export let data;

    let label = '.';
    $: trimmedLabel = label.trim();

    let script = '';
    let template = '';

    let scriptTextarea: HTMLTextAreaElement;
    let templateTextarea: HTMLTextAreaElement;

    onMount(async () => {
        label = data.card.label;
        script = data.card.script;
        template = data.card.template;

        await tick();

        autosize.update(scriptTextarea);
        autosize.update(templateTextarea);
    });

    $: isLabelChanged = trimmedLabel !== data.card.label;
    $: isLabelValid = trimmedLabel.length > 0;

    $: isScriptChanged = script !== data.card.script;
    $: isTemplateChanged = template !== data.card.template;

    $: canSave = (isLabelChanged || isScriptChanged || isTemplateChanged) && isLabelValid;

    const saveChanges = async () => {
        await invoke('card_modify', {
            id: data.card.id,
            action: {
                Edit: {
                    label: isLabelChanged ? trimmedLabel : null,
                    script: isScriptChanged ? script : null,
                    template: isTemplateChanged ? template : null
                }
            }
        });

        await goto(`/pack/${data.card.pack_id}`);
    };
</script>

<AppBar>
    <svelte:fragment slot="lead">
        <a href="/pack/{data.card.pack_id}">
            <i class="fa-solid fa-arrow-left fa-lg" />
        </a>
    </svelte:fragment>

    <h1 class="text-xl font-semibold">Edit Card</h1>

    <svelte:fragment slot="trail">
        {#if canSave}
            <button class="chip variant-filled" on:click={saveChanges}>Save</button>
        {/if}
    </svelte:fragment>
</AppBar>

<div class="space-y-4 p-4">
    <label class="label">
        <span class={isLabelValid ? '' : 'text-error-500'}>Card Label</span>
        <input
            type="text"
            class="input variant-form-material {isLabelValid ? '' : 'input-error'}"
            placeholder="Enter a Label..."
            bind:value={label}
        />
    </label>

    <label class="label">
        <span>Script</span>
        <textarea
            class="textarea variant-form-material font-mono"
            rows={1}
            bind:this={scriptTextarea}
            bind:value={script}
            use:autosize
        />
    </label>

    <label class="label">
        <span>Template</span>
        <textarea
            class="textarea variant-form-material font-mono"
            rows={1}
            bind:this={templateTextarea}
            bind:value={template}
            use:autosize
        />
    </label>
</div>
