<script lang="ts">
    import { goto, invalidateAll } from '$app/navigation';
    import { page } from '$app/stores';
    import type { Pack } from '@bindings/Pack';
    import { deletePack } from '$lib/commands';
    import { conditionalClass } from '$lib/styling';
    import ModalController from '$lib/ModalController.svelte';
    import RenamePack from './RenamePack.svelte';

    export let pack: Pack;

    let hovering = false;

    $: href = `/pack/${pack.id}`;
    $: selected = $page.url.pathname.startsWith(href);

    const remove = async () => {
        if (selected) {
            await goto('/pack');
        }

        await deletePack(pack.id);

        await invalidateAll();
    };

    $: linkClass = conditionalClass(selected, {
        base: 'w-full flex gap-2 font-semibold py-1 px-3 rounded cursor-default',
        on: 'bg-indigo-500 hover:bg-indigo-600 shadow text-white',
        off: 'hover:bg-slate-200'
    });
</script>

<ModalController let:active let:open let:close>
    <a
        {href}
        class={linkClass}
        on:mouseenter={() => (hovering = true)}
        on:mouseleave={() => (hovering = false)}
    >
        <span class="text-ellipsis overflow-hidden flex-grow">
            {pack.title}
        </span>
        {#if hovering || selected || active}
            <button class="flex-none" on:click|preventDefault={open}>
                <i class="fa-solid fa-pen text-sm" />
            </button>
        {/if}
    </a>

    <RenamePack {close} {pack} slot="modal" />
</ModalController>
