<script lang="ts">
    import { goto, invalidateAll } from '$app/navigation';
    import RenamePack from './RenamePack.svelte';
    import ContextMenu from '$lib/ContextMenu.svelte';
    import MenuButton from '$lib/MenuButton.svelte';
    import { invoke } from '$lib/commands';
    import type { PackSummary } from '@bindings/PackSummary';
    import QuickStudy from './QuickStudy.svelte';
    import { modals } from '$lib/modals';
    import ConfirmDelete from './ConfirmDelete.svelte';

    export let pack: PackSummary;

    let hovering = false;

    $: href = `/pack/${pack.id}`;

    export let activePack: number | null;

    $: selected = pack.id === activePack;

    let menu: ContextMenu;

    const remove = async () => {
        if (selected) {
            await goto('/pack');
        }

        await invoke('modify_pack', { id: pack.id, action: 'Delete' });
        await invalidateAll();
    };

    const quickStudy = async () => {
        modals.add(QuickStudy, { packId: pack.id });
    };
</script>

<a
    {href}
    class="flex w-full cursor-default gap-2 rounded py-1 px-3 font-semibold
        {selected ? 'bg-violet-500 text-white shadow hover:bg-violet-600' : 'hover:bg-slate-200'}"
    on:mouseenter={() => (hovering = true)}
    on:mouseleave={() => (hovering = false)}
    on:contextmenu|preventDefault={menu.onContextMenu}
    on:click={() => (activePack = pack.id)}
>
    <span class="flex-grow overflow-hidden text-ellipsis">
        {pack.title}
    </span>
    {#if hovering || selected}
        <button class="flex-none" on:click|preventDefault|stopPropagation={quickStudy}>
            <i class="fa-solid fa-bolt-lightning fa-sm" />
        </button>
    {/if}
</a>

<ContextMenu bind:this={menu}>
    <MenuButton on:click={() => modals.add(RenamePack, { pack })} label="Rename" icon="pen" />
    <MenuButton
        on:click={() => modals.add(ConfirmDelete, { confirm: remove })}
        label="Delete"
        danger
        icon="trash"
    />
</ContextMenu>
