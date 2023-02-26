<script lang="ts">
    import CreatePack from '@lib/CreatePack.svelte';
    import PackSelector from '@lib/PackSelector.svelte';
    import { packs } from '@lib/packs';
    import Transition from '@lib/Transition.svelte';
    import type { LayoutData } from './$types';

    export let data: LayoutData;
</script>

<div class="sidebar">
    <CreatePack />
    <PackSelector packs={$packs.list} disabled={$packs.loading} />
</div>

<main class="content">
    <Transition key={data.href}>
        <slot />
    </Transition>
</main>

<style lang="postcss">
    .sidebar {
        width: min(40vw, 200px);
        position: absolute;
    }

    .content {
        margin-left: min(40vw, 200px);
        font-size: 18px;
    }
</style>
