<script lang="ts">
    import { activeTab, type Tab } from '$lib/routing/tabs';

    export let tab: Tab;
    export let icon: string;
    export let href: string;

    $: active = $activeTab === tab;
</script>

<a
    class="relative flex aspect-square w-full items-center justify-center p-3 hover:bg-slate-100
        {active ? 'bg-slate-200 text-slate-900' : 'text-slate-800'}"
    {href}
>
    <i class="fa-solid fa-{icon} fa-xl" />
</a>
