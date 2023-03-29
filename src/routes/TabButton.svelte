<script lang="ts">
    import { activeTab, type Tab } from '$lib/routing/tabs';

    export let tab: Tab;
    export let label: string;
    export let href: string;

    $: active = $activeTab === tab;
</script>

<a
    class="hover:bg-slate-1000 rounded
        {active ? 'bg-white' : ''}"
    {href}
>
    {label}
</a>
