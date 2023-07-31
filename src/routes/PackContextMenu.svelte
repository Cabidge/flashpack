<script lang="ts">
    import { invoke } from '$lib/commands';
    import type { Pack } from '@bindings/Pack';
    import { modalStore } from '@skeletonlabs/skeleton';

    export let pack: Pack;
    export let popup: string;
</script>

<div class="card p-4" data-popup={popup}>
    <div class="arrow bg-surface-100-800-token" />
    <div class="flex flex-col">
        <button
            class="btn"
            on:click={() => {
                modalStore.trigger({
                    type: 'prompt',
                    title: 'Rename Pack',
                    buttonTextSubmit: 'Save',
                    value: pack.title,
                    valueAttr: {
                        type: 'text',
                        placeholder: 'New Title',
                        required: true
                    },
                    response: (newTitle) => {
                        invoke('pack_modify', {
                            id: pack.id,
                            action: { Rename: newTitle }
                        });
                    }
                });
            }}
        >
            Rename
        </button>
        <button
            class="btn"
            on:click={() => {
                modalStore.trigger({
                    type: 'confirm',
                    title: 'Delete Pack',
                    body: `Are you sure you want to delete '${pack.title}'?`,
                    buttonTextConfirm: 'Delete',
                    response: (doDelete) => {
                        if (doDelete) {
                            invoke('pack_modify', {
                                id: pack.id,
                                action: 'Delete'
                            });
                        }
                    }
                });
            }}
        >
            Delete
        </button>
    </div>
</div>
