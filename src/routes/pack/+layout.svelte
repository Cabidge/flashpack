<script lang="ts">
    import CreatePack from '@lib/CreatePack.svelte';
    import PackSelector from '@lib/PackSelector.svelte';
    import Transition from '@lib/Transition.svelte';
    import type { LayoutData } from './$types';

    export let data: LayoutData;

    $: packs = data.packs;
</script>

<div class="flex flex-row h-full">
    <div class="w-1/3 max-w-[16rem] float-left overflow-x-hidden">
        <CreatePack />
        <PackSelector {packs} />
    </div>

    <main>
        <Transition key={data.href}>
            <slot />
        </Transition>
    </main>
</div>