<script lang="ts">
    import { goto, invalidateAll } from '$app/navigation';
    import type { Pack } from '@bindings/Pack';
    import { deletePack } from './commands';
    import SmartLink from './SmartLink.svelte';

    export let pack: Pack;

    let hovering = false;

    const remove = async () => {
        await deletePack(pack.id);
        await goto('/pack');

        await invalidateAll();
    };
</script>

<div
    class="w-full flex items-stretch relative"
    on:mouseenter={() => (hovering = true)}
    on:mouseleave={() => (hovering = false)}
>
    <SmartLink
        href="/pack/{pack.id}"
        strict={false}
        styling={{
            base: 'w-full font-semibold py-1 px-4 rounded',
            unselected: 'hover:bg-slate-200',
            selected: 'bg-indigo-500 hover:bg-indigo-600 shadow text-white'
        }}
    >
        {pack.title}
        {#if hovering}
            <button
                class="absolute right-3 top-0 bottom-0 hover:bg-slate-300"
                on:click|preventDefault={remove}>del</button
            >
        {/if}
    </SmartLink>
</div>
