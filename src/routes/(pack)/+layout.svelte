<script lang="ts">
    import CreatePack from './CreatePack.svelte';
    import ModalController from '$lib/ModalController.svelte';
    import PackSelector from './PackSelector.svelte';
    import Transition from '$lib/Transition.svelte';
    import type { LayoutData } from './$types';
    import { onMount } from 'svelte';
    import { activeTab } from '$lib/routing/tabs';

    export let data: LayoutData;

    let search = '';
    $: query = search.toLowerCase();

    $: ({ activePack, packs } = data);

    $: filteredPacks =
        search === '' ? packs : packs.filter((pack) => pack.title.toLowerCase().includes(query));

    onMount(() => ($activeTab = 'pack'));
</script>

<div class="flex h-full flex-row">
    <div
        class="float-left flex w-1/3 min-w-[14rem] max-w-[18rem] flex-col gap-3 overflow-x-hidden bg-slate-100 p-4 shadow"
    >
        <div class="flex gap-2">
            <input class="w-full rounded pl-2 shadow" bind:value={search} placeholder="Search..." />

            <ModalController title="Create a Pack" let:open let:close>
                <button
                    class="aspect-square w-9 flex-none rounded bg-indigo-500 text-center text-xl font-semibold text-white shadow hover:bg-indigo-600"
                    on:click={open}
                >
                    +
                </button>

                <CreatePack slot="modal" {close} />
            </ModalController>
        </div>

        <div class="border-b-2" />

        <PackSelector packs={filteredPacks} activePack={$activePack} />
    </div>

    <main class="h-full w-full overflow-auto px-6 py-4">
        <slot />
    </main>
</div>
