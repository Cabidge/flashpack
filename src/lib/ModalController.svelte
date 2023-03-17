<script lang="ts">
    import Modal from './Modal.svelte';

    let active = false;
    export let title: string;

    const open = () => (active = true);
    const close = () => (active = false);
</script>

<slot {active} {open} {close} />

<Modal {title} bind:active>
    <slot name="modal" />
</Modal>
