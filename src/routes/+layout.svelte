<script lang="ts">
    import { menuStatus } from '$lib/context_menu';
    import '../app.postcss';
    import TabButton from './TabButton.svelte';

    type TabInfo = {
        label: string;
        href: string;
    };
</script>

<div class="flex h-screen flex-row">
    <nav class="item-stretch flex flex-col gap-2 bg-slate-300 p-2">
        <ul>
            <li><TabButton tab="pack" label="Pack" href="/pack" /></li>
            <li><TabButton tab="study" label="Study" href="/study" /></li>
        </ul>
    </nav>

    <div class="h-full w-full overflow-hidden">
        <slot />
    </div>
</div>

<svelte:body
    on:click={() => {
        menuStatus.close();
    }}
/>
