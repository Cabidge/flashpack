<script lang="ts">
    import { activeTab, type Tab } from '$lib/routing/tabs';

    export let tab: Tab;
    export let icon: string;
    export let href: string;

    $: active = $activeTab === tab;
</script>

<a
    class="relative flex aspect-square w-full items-center justify-center p-3 text-slate-800 hover:bg-slate-200"
    {href}
>
    <i class="fa-solid fa-{icon} fa-xl" />
    {#if active}
        <div class="absolute right-1 left-1 bottom-1 h-1 rounded bg-slate-100" />
    {/if}
</a>
