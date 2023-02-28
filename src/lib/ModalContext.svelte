<script lang="ts">
    import Modal from './Modal.svelte';

    let active = false;

    const open = () => (active = true);
    const close = () => (active = false);
</script>

<slot {active} {open} {close} />

<Modal bind:active>
    <slot name="modal" />
</Modal>
