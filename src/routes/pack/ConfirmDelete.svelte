<script lang="ts">
    import { getModalContext } from '$lib/modals';

    export let confirm: () => void;

    const { close } = getModalContext();
</script>

<button
    on:click={() => {
        close();
        confirm();
    }}
>
    Yes
</button>
<button on:click={close}>Cancel</button>
