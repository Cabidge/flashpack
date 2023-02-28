<script lang="ts">
    import CreatePack from '@lib/CreatePack.svelte';
    import ModalContext from '@lib/ModalContext.svelte';
    import PackSelector from '@lib/PackSelector.svelte';
    import Transition from '@lib/Transition.svelte';
    import type { LayoutData } from './$types';

    export let data: LayoutData;

    $: packs = data.packs;
</script>

<div class="flex flex-row h-full">
    <div class="w-1/3 max-w-[16rem] p-4 float-left overflow-x-hidden shadow flex flex-col gap-3">
        <ModalContext let:open let:close>
            <button
                class="rounded bg-indigo-500 text-white hover:bg-indigo-600 text-center w-full py-2 shadow font-semibold"
                on:click={open}
            >
                Create Pack
            </button>

            <CreatePack slot="modal" on:close={close} />
        </ModalContext>

        <div class="border-b-2" />

        <PackSelector {packs} />
    </div>

    <main class="p-2">
        <Transition key={data.href}>
            <slot />
        </Transition>
    </main>
</div>
