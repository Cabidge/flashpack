<script lang="ts">
    import CreatePack from './CreatePack.svelte';
    import ModalController from '$lib/ModalController.svelte';
    import PackSelector from './PackSelector.svelte';
    import Transition from '$lib/Transition.svelte';
    import type { LayoutData } from './$types';

    export let data: LayoutData;

    let search = '';
    $: query = search.toLowerCase();

    $: packs = data.packs;

    $: filteredPacks =
        search === '' ? packs : packs.filter((pack) => pack.title.toLowerCase().includes(query));
</script>

<div
    class="float-left flex w-1/3 max-w-[16rem] flex-col gap-3 overflow-x-hidden bg-slate-100 p-4 shadow"
>
    <div class="flex gap-2">
        <input class="min-w-0 rounded pl-2 shadow" bind:value={search} placeholder="Search..." />

        <ModalController let:open let:close>
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

    <PackSelector packs={filteredPacks} />
</div>

<main class="px-6 py-4">
    <Transition key={data.href}>
        <slot />
    </Transition>
</main>
