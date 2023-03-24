<script lang="ts">
    import { invoke } from '$lib/commands';
    import type { DealerFilter } from '@bindings/DealerFilter';
    import type { FilterSummary } from '@bindings/FilterSummary';
    import type { ModifyDealer } from '@bindings/ModifyDealer';
    import { createEventDispatcher, onMount } from 'svelte';

    export let dealerFilters: DealerFilter[];

    $: oldFilterIds = new Set(dealerFilters.map((filter) => filter.id));

    let selection: FilterSummary[];
    let filters: FilterSummary[] = [];

    onMount(async () => {
        filters = await invoke('list_filters');
        selection = filters.filter(({ id }) => oldFilterIds.has(id));
    });

    const diffChanges = () => {
        const oldFilterIds = new Set(dealerFilters.map((filter) => filter.id));
        const newFilterIds = new Set(selection.map((filter) => filter.id));

        return filters.reduce((modifications: ModifyDealer[], { id }) => {
            const oldHas = oldFilterIds.has(id);
            const newHas = newFilterIds.has(id);

            if (oldHas && !newHas) {
                modifications.push({ RemoveFilter: id });
            } else if (newHas && !oldHas) {
                modifications.push({ AddFilter: id });
            }

            return modifications;
        }, []);
    };

    const dispatch = createEventDispatcher<{ save: ModifyDealer[] }>();
</script>

<div class="w-[80vw] max-w-md">
    <select class="w-full" multiple bind:value={selection}>
        {#each filters as filter (filter.id)}
            <option value={filter}>{filter.label}</option>
        {/each}
    </select>

    <br />

    <button on:click={() => dispatch('save', diffChanges())}>save</button>
</div>
