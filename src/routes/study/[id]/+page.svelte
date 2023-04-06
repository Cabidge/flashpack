<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import ModalController from '$lib/ModalController.svelte';
    import type { PageData } from './$types';
    import EditFilters from './EditFilters.svelte';
    import EditWeight from './EditWeight.svelte';

    export let data: PageData;

    $: ({ id, dealer } = data);

    $: filterEntries = Object.entries(dealer.filters);
</script>

<h1>{dealer.title}</h1>

{#if dealer.invalid_filters.length > 0}
    <p>There are one or more invalid filters selected which prevents practice...</p>
{:else if filterEntries.length > 0}
    <a href="{data.href}/practice">Begin Study</a>
{/if}

<ModalController title="Select Filters" let:open let:close>
    <button on:click={open}>Edit Filters</button>

    <EditFilters
        on:save={async (e) => {
            close();

            for (const action of e.detail) {
                await invoke('modify_dealer', { id, action });
            }

            await invalidateAll();
        }}
        dealerFilters={dealer.filters}
        slot="modal"
    />
</ModalController>

{#if filterEntries.length === 0}
    <p>No filters selected...</p>
{:else}
    <ul>
        {#each filterEntries as [packTitle, filterGroup] (packTitle)}
            <li>
                <span class="font-semibold">
                    {packTitle}
                </span>
                <ul>
                    {#each filterGroup as filter (filter.summary.id)}
                        <li>
                            <ModalController title="Edit Weight" let:open let:close>
                                <button
                                    on:click={open}
                                    class="group"
                                    class:text-red-500={dealer.invalid_filters.includes(
                                        filter.summary.id
                                    )}
                                >
                                    {filter.summary.label}
                                    <span
                                        class="text-xs text-violet-600 {filter.weight === 1
                                            ? 'hidden group-hover:inline'
                                            : ''}"
                                    >
                                        (x{filter.weight})
                                    </span>
                                </button>

                                <EditWeight
                                    slot="modal"
                                    weight={filter.weight}
                                    on:save={async (e) => {
                                        close();
                                        await invoke('modify_dealer', {
                                            id,
                                            action: { SetWeight: [filter.summary.id, e.detail] }
                                        });
                                        await invalidateAll();
                                    }}
                                />
                            </ModalController>
                        </li>
                    {/each}
                </ul>
            </li>
        {/each}
    </ul>
{/if}
