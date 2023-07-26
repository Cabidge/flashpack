<script lang="ts">
    import { invoke } from '$lib/commands.js';
    import SlideView from '$lib/components/SlideView.svelte';
    import { AppBar, modalStore } from '@skeletonlabs/skeleton';

    export let data;
</script>

<AppBar>
    <svelte:fragment slot="lead">
        <a href="/">
            <i class="fa-solid fa-arrow-left fa-lg" />
        </a>
    </svelte:fragment>

    <h1 class="text-xl font-semibold">
        {data.pack.title}
    </h1>

    <svelte:fragment slot="trail">
        <button
            class="chip variant-filled"
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
    </svelte:fragment>
</AppBar>

<div class="p-4">
    <ul class="list">
        {#each data.cards as card (card.id)}
            <li>
                <a href="/card/{card.id}" class="chip variant-filled">edit</a>
                <button
                    class="btn variant-filled-surface"
                    on:click={() => {
                        modalStore.trigger({
                            type: 'component',
                            component: {
                                ref: SlideView,
                                props: { script: card.script, template: card.template }
                            }
                        });
                    }}
                >
                    {card.label}
                </button>
            </li>
        {/each}
    </ul>
</div>
