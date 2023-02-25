<script lang="ts">
    import '../app.postcss';

    import type { LayoutData } from './$types';
    import CreatePack from './CreatePack.svelte';
    import PackSelector from './PackSelector.svelte';

    export let data: LayoutData;

    $: packs = data.packs;
</script>

<nav>
    <a href="/">packs</a>
</nav>

<div class="sidebar">
    <PackSelector bind:packs />
    <CreatePack />
</div>

<main class="content">
    <slot />
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
