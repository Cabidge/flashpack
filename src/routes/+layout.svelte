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
    <nav class="bg-slate-300">
        <ul class="flex flex-col items-stretch">
            <li class="flex"><TabButton tab="pack" icon="layer-group" href="/pack" /></li>
            <li class="flex"><TabButton tab="study" icon="book" href="/study" /></li>
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
