<script lang="ts">
    import { goto, invalidateAll } from '$app/navigation';
    import { page } from '$app/stores';
    import type { Pack } from '@bindings/Pack';
    import { conditionalClass } from '$lib/styling';
    import ModalController from '$lib/ModalController.svelte';
    import RenamePack from './RenamePack.svelte';
    import ContextMenu from '$lib/ContextMenu.svelte';
    import MenuButton from '$lib/MenuButton.svelte';
    import Modal from '$lib/Modal.svelte';
    import { invoke } from '$lib/commands';

    export let pack: Pack;

    let hovering = false;

    $: href = `/pack/${pack.id}`;
    $: selected = $page.url.pathname.startsWith(href);

    $: linkClass = conditionalClass(selected, {
        base: 'w-full flex gap-2 font-semibold py-1 px-3 rounded cursor-default',
        on: 'bg-indigo-500 hover:bg-indigo-600 shadow text-white',
        off: 'hover:bg-slate-200'
    });

    let menu: ContextMenu;
    let deleteModal: Modal;

    const remove = async () => {
        deleteModal.close();

        if (selected) {
            await goto('/pack');
        }

        await invoke('deletePack', { id: pack.id });
        await invalidateAll();
    };
</script>

<ModalController let:active let:open let:close>
    <a
        {href}
        class={linkClass}
        on:mouseenter={() => (hovering = true)}
        on:mouseleave={() => (hovering = false)}
        on:contextmenu|preventDefault={(e) => {
            menu.select(e.clientX, e.clientY);
        }}
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
    <MenuButton label="Quick Study" icon="book" />
    <MenuButton on:click={deleteModal.open} label="Delete" danger icon="trash" />
</ContextMenu>

<Modal bind:this={deleteModal}>
    <span>Delete {pack.title}?</span>
    <button on:click={remove}>yes</button>
    <button on:click={deleteModal.close}>cancel</button>
</Modal>
