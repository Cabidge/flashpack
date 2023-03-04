<script lang="ts">
    import CreatePack from './CreatePack.svelte';
    import ModalController from '$lib/ModalController.svelte';
    import PackSelector from './PackSelector.svelte';
    import Transition from '$lib/Transition.svelte';
    import type { LayoutData } from './$types';

    export let data: LayoutData;

    $: packs = data.packs;
</script>

<div
    class="float-left flex w-1/3 max-w-[16rem] flex-col gap-3 overflow-x-hidden bg-slate-100 p-4 shadow"
>
    <ModalController let:open let:close>
        <button
            class="w-full rounded bg-indigo-500 py-2 text-center font-semibold text-white shadow hover:bg-indigo-600"
            on:click={open}
        >
            Create Pack
        </button>

        <CreatePack slot="modal" {close} />
    </ModalController>

    <div class="border-b-2" />

    <PackSelector {packs} />
</div>

<main class="p-2">
    <Transition key={data.href}>
        <slot />
    </Transition>
</main>
