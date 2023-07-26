<script lang="ts">
    import { invoke } from '$lib/commands.js';
    import { modalStore } from '@skeletonlabs/skeleton';

    export let data;
</script>

<a href="/">Return to Packs</a>

<h1>{data.pack.title}</h1>

<button
    class="btn variant-filled"
    on:click={() => {
        modalStore.trigger({
            type: 'prompt',
            title: 'Add a Card',
            valueAttr: { type: 'text', placeholder: 'Label', required: true },
            response: (label) => invoke('card_create', { packId: data.pack.id, label })
        });
    }}
>
    Add a Card
</button>

<nav class="list-nav">
    <ul>
        {#each data.cards as card (card.id)}
            <li>
                <a href="/card/{card.id}">
                    {card.label}
                </a>
            </li>
        {/each}
    </ul>
</nav>
