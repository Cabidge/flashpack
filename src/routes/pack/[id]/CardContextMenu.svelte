<script lang="ts">
    import { goto } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import type { Card } from '@bindings/Card';
    import { modalStore } from '@skeletonlabs/skeleton';

    export let card: Card;
    export let popup: string;

    const handleRename = () => {
        modalStore.trigger({
            type: 'prompt',
            title: 'Rename Card',
            buttonTextSubmit: 'Save',
            value: card.label,
            valueAttr: {
                type: 'text',
                placeholder: 'New Label',
                required: true
            },
            response: (newLabel) => {
                invoke('card_modify', {
                    id: card.id,
                    action: {
                        Edit: {
                            label: newLabel,
                            script: null,
                            template: null
                        }
                    }
                });
            }
        });
    };

    const handleDuplicate = async () => {
        const shouldDuplicate = await new Promise((resolve) => {
            modalStore.trigger({
                type: 'confirm',
                title: 'Duplicate Card',
                body: `Do you want to make a copy of '${card.label}'?`,
                response: resolve
            });
        });

        if (!shouldDuplicate) {
            return;
        }

        const id = await invoke('card_create', {
            packId: card.pack_id,
            label: `${card.label} (Copy)`
        });

        await invoke('card_modify', {
            id,
            action: {
                Edit: {
                    label: null,
                    script: card.script,
                    template: card.template
                }
            }
        });
    };

    const handleDelete = () => {
        modalStore.trigger({
            type: 'confirm',
            title: 'Delete Card',
            body: `Are you sure you want to delete '${card.label}'?`,
            buttonTextConfirm: 'Delete',
            response: (doDelete) => {
                if (doDelete) {
                    invoke('card_modify', {
                        id: card.id,
                        action: 'Delete'
                    });
                }
            }
        });
    };
</script>

<div class="card z-10 p-4" data-popup={popup}>
    <div class="arrow bg-surface-100-800-token" />
    <div class="flex flex-col">
        <a href="/card/{card.id}" class="btn">Edit</a>
        <button class="btn" on:click={handleRename}>Rename</button>
        <button class="btn" on:click={handleDuplicate}>Duplicate</button>
        <button class="btn" on:click={handleDelete}>Delete</button>
    </div>
</div>
