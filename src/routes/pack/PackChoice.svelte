<script lang="ts">
    import { goto, invalidateAll } from '$app/navigation';
    import { page } from '$app/stores';
    import ModalController from '$lib/ModalController.svelte';
    import RenamePack from './RenamePack.svelte';
    import ContextMenu from '$lib/ContextMenu.svelte';
    import MenuButton from '$lib/MenuButton.svelte';
    import Modal from '$lib/Modal.svelte';
    import { invoke } from '$lib/commands';
    import type { PackSummary } from '@bindings/PackSummary';

    export let pack: PackSummary;

    let hovering = false;

    $: href = `/pack/${pack.id}`;

    let selected: boolean;
    $: {
        const path = $page.url.pathname.split('/');
        selected = path[1] === 'pack' && path[2] === pack.id.toString();
    }

    let menu: ContextMenu;
    let deleteModal: Modal;

    const remove = async () => {
        deleteModal.close();

        if (selected) {
            await goto('/pack');
        }

        await invoke('modify_pack', { id: pack.id, action: 'Delete' });
        await invalidateAll();
    };

    const quickStudy = async () => {
        // TODO
        console.log('Study not ready...');
    };
</script>

<ModalController title="Rename Pack" let:active let:open let:close>
    <a
        {href}
        class="flex w-full cursor-default gap-2 rounded py-1 px-3 font-semibold
            {selected
            ? 'bg-indigo-500 text-white shadow hover:bg-indigo-600'
            : 'hover:bg-slate-200'}"
        on:mouseenter={() => (hovering = true)}
        on:mouseleave={() => (hovering = false)}
        on:contextmenu|preventDefault={menu.onContextMenu}
    >
        <span class="flex-grow overflow-hidden text-ellipsis">
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

<ContextMenu bind:this={menu}>
    <MenuButton on:click={quickStudy} label="Quick Study" icon="book" />
    <MenuButton on:click={deleteModal.open} label="Delete" danger icon="trash" />
</ContextMenu>

<!-- TODO: Fix delete transition acting funky when this Modal is transitioning out -->
<Modal title="Delete {pack.title}?" bind:this={deleteModal}>
    <button on:click={remove}>yes</button>
    <button on:click={deleteModal.close}>cancel</button>
</Modal>
