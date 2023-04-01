<script lang="ts">
    import ModalController from './ModalController.svelte';

    export let tags: string[];
</script>

<ModalController title="Tags" let:open>
    <button on:click={open}>Show Tags</button>

    <ul slot="modal" class="flex flex-row gap-2">
        {#each tags as tag (tag)}
            <li class="rounded-full bg-slate-100 px-2">{tag}</li>
        {/each}
    </ul>
</ModalController>
