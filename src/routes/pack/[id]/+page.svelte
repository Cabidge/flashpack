<script lang="ts">
    import { goto } from '$app/navigation';
    import { invoke } from '$lib/commands.js';
    import ModalCardPreview from '$lib/components/ModalCardPreview.svelte';
    import { AppBar, modalStore, popup } from '@skeletonlabs/skeleton';
    import CardContextMenu from './CardContextMenu.svelte';
    import ReturnLinkButton from '$lib/components/ReturnLinkButton.svelte';
    import VerticalDots from '~icons/mdi/dots-vertical';

    export let data;

    $: hasCards = data.cards.length > 0;
</script>

<AppBar>
    <svelte:fragment slot="lead">
        <ReturnLinkButton href="/" />
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
                    valueAttr: { type: 'text', placeholder: 'Label' },
                    response: async (label) => {
                        const id = await invoke('card_create', { packId: data.pack.id, label });
                        await goto(`/card/${id}`);
                    }
                });
            }}
        >
            Add a Card
        </button>
    </svelte:fragment>
</AppBar>

<div class="space-y-4 p-4">
    <section class="space-y-4 border border-surface-500 p-4 rounded-container-token">
        <p class="font-semibold">Cards</p>

        {#if hasCards}
            <nav class="list-nav">
                <ul>
                    {#each data.cards as card (card.id)}
                        {@const hasLabel = card.label.length > 0}
                        <li>
                            <button
                                class="w-full"
                                on:click={() => {
                                    modalStore.trigger({
                                        type: 'component',
                                        component: {
                                            ref: ModalCardPreview,
                                            props: { card }
                                        }
                                    });
                                }}
                            >
                                <span class="flex-auto text-left {hasLabel ? '' : 'italic'}">
                                    {hasLabel ? card.label : card.template}
                                </span>
                                <button
                                    class="btn-icon"
                                    on:click|stopPropagation={() => {}}
                                    use:popup={{
                                        event: 'click',
                                        target: `popup-${card.id}`,
                                        placement: 'bottom'
                                    }}
                                >
                                    <VerticalDots class="scale-150" />
                                </button>
                            </button>
                        </li>
                        <CardContextMenu {card} popup="popup-{card.id}" />
                    {/each}
                </ul>
            </nav>
        {:else}
            <p>No cards yet, click 'Add a Card.'</p>
        {/if}
    </section>

    {#if hasCards}
        {#if data.cards.length >= 2}
            <a class="btn variant-filled-primary w-full" href="/pack/{data.pack.id}/practice">
                Flash Study
            </a>
            <a class="btn variant-ghost w-full" href="/pack/{data.pack.id}/endless">
                Endless Study
            </a>
        {:else}
            <button class="btn variant-filled-primary w-full" disabled>
                Add More Cards to Begin Practice
            </button>
        {/if}
    {/if}
</div>
