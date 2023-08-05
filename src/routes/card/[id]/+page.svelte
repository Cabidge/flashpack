<script lang="ts">
    import { goto } from '$app/navigation';
    import { invoke } from '$lib/commands.js';
    import ReturnLinkButton from '$lib/components/ReturnLinkButton.svelte';
    import { AppBar } from '@skeletonlabs/skeleton';
    import { onMount, tick } from 'svelte';
    import autosize from 'svelte-autosize';
    import HighlightTextArea from './HighlightTextArea.svelte';

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
    $: isScriptChanged = script !== data.card.script;
    $: isTemplateChanged = template !== data.card.template;

    $: canSave = isLabelChanged || isScriptChanged || isTemplateChanged;

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
        <ReturnLinkButton href="/pack/{data.card.pack_id}" />
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
        <span>Card Label</span>
        <input
            type="text"
            class="input variant-form-material"
            placeholder="Enter a Label..."
            bind:value={label}
        />
    </label>

    <!--TODO: custom highlighter for rhai-->
    <label class="label" for={undefined}>
        <span>Script</span>
        <HighlightTextArea
            bind:textarea={scriptTextarea}
            bind:value={script}
            language="javascript"
        />
    </label>

    <!--TODO: custom highlighter for the custom templating language-->
    <label class="label" for={undefined}>
        <span>Template</span>
        <HighlightTextArea
            bind:textarea={templateTextarea}
            bind:value={template}
            language="jinja"
        />
    </label>
</div>
