<script lang="ts">
    import { invoke } from '$lib/commands';
    import type { Card } from '@bindings/Card';

    export let card: Card;

    let rendered: string = '';

    $: invoke('generate_card_slides', {
        script: card.script,
        template: card.template
    }).then((slides) => (rendered = slides.join('<hr>')));
</script>

<div class="card w-modal p-4 shadow-xl">
    <header class="mb-4 w-full">
        <span class="text-xl font-semibold">
            {card.label}
        </span>
        <button class="chip variant-filled float-right" on:click={() => (card = card)}>
            refresh
        </button>
    </header>

    <div class="border border-surface-500 p-4 rounded-container-token">
        {#await rendered then rendered}
            <div class="prose dark:prose-invert prose-hr:my-4">
                {@html rendered}
            </div>
        {/await}
    </div>
</div>
