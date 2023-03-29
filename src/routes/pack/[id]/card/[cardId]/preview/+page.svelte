<script lang="ts">
    import PromptView from '$lib/PromptView.svelte';
    import type { PageData } from './$types';

    export let data: PageData;

    $: ({ packHref, cardHref, prompt } = data);
</script>

<a href={packHref}>Return to Pack</a>
<a href={cardHref}>Edit</a>

<PromptView {prompt} />
