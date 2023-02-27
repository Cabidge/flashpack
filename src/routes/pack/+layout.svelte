<script lang="ts">
    import CreatePack from '@lib/CreatePack.svelte';
    import PackSelector from '@lib/PackSelector.svelte';
    import Transition from '@lib/Transition.svelte';
    import type { LayoutData } from './$types';

    export let data: LayoutData;

    $: packs = data.packs;
</script>

<div class="flex flex-row h-full">
    <div class="w-1/3 max-w-[16rem] p-4 float-left overflow-x-hidden shadow flex flex-col gap-3">
        <CreatePack />
        <div class="border-b-2"></div>
        <PackSelector {packs} />
    </div>

    <main class="p-2">
        <Transition key={data.href}>
            <slot />
        </Transition>
    </main>
</div>