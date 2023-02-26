<script lang="ts">
    import CreatePack from '@lib/CreatePack.svelte';
    import PackSelector from '@lib/PackSelector.svelte';
    import { packs } from '@lib/packs';
    import Transition from '@lib/Transition.svelte';
    import type { LayoutData } from './$types';

    export let data: LayoutData;
</script>

<div class="w-1/4 max-w-[32rem] float-left overflow-x-hidden">
    <CreatePack />
    <PackSelector packs={$packs.list} disabled={$packs.loading} />
</div>

<main>
    <Transition key={data.href}>
        <slot />
    </Transition>
</main>
