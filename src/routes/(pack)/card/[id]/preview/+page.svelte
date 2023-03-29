<script lang="ts">
    import PromptView from '$lib/PromptView.svelte';
    import type { PageData } from './$types';

    export let data: PageData;

    $: ({ id, card, prompt } = data);
</script>

<a href="/pack/{card.pack_id}">Return to Pack</a>
<a href="/card/{id}">Edit</a>

<PromptView {prompt} />
