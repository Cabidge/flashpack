<script lang="ts">
    import { onMount } from 'svelte';
    import { setModalContext, type ModalContent, modals } from './modals';
    import { fade, fly } from 'svelte/transition';

    export let content: ModalContent;

    let closed = false;

    const closeContent = () => {
        if (closed) {
            return;
        }

        closed = true;
        content.close();
    };

    let dialog: HTMLDialogElement;

    onMount(() => {
        dialog.showModal();
    });

    setModalContext({
        close: () => closeContent(),
    });
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<dialog
    bind:this={dialog}
    on:cancel|preventDefault={closeContent}
    on:close={closeContent}
    on:click|stopPropagation={(e) => {
        let rect = dialog.getBoundingClientRect();
        let isInDialog =
            rect.top <= e.clientY &&
            e.clientY <= rect.top + rect.height &&
            rect.left <= e.clientX &&
            e.clientX <= rect.left + rect.width;

        if (!isInDialog) {
            closeContent();
        }
    }}
    class="rounded shadow backdrop:fixed backdrop:inset-0 backdrop:bg-black backdrop:bg-opacity-50
        {closed ? 'backdrop:hidden' : ''}"
    transition:fly={{ duration: 300, y: 10 }}
>
    <svelte:component this={content.component} {...content.props} />
</dialog>
