<script lang="ts">
    import { onMount } from 'svelte';
    import { setModalContext, type ModalContent, modals } from './modals';
    import { fade, fly } from 'svelte/transition';
    import TagInput from './TagInput.svelte';

    export let content: ModalContent;

    let dialog: HTMLDialogElement;

    onMount(() => {
        dialog.showModal();
    });

    setModalContext({
        close: () => {
            dialog.close();
        }
    });
</script>

<dialog
    bind:this={dialog}
    on:cancel|preventDefault={content.close}
    on:close={() =>{
        content.close();
        dialog.showModal();
    }}
    class="rounded shadow backdrop:fixed backdrop:inset-0 backdrop:bg-black backdrop:bg-opacity-50"
    transition:fly={{ duration: 300, y: 10 }}
>
    <svelte:component this={content.component} {...content.props} />
</dialog>
