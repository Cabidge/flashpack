<script lang="ts">
    import { goto, invalidateAll } from '$app/navigation';
    import { page } from '$app/stores';
    import { addCard } from '@lib/commands';

    $: id = $page.params.id;

    let front = '';
    let back = '';

    const submit = async () => {
        const card = { pack_id: id, front, back };

        try {
            await addCard(card);
        } catch (err) {
            console.log(err);
        }

        const url = `/pack/${id}`;

        await goto(url);
        await invalidateAll();
    };
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={front} placeholder="front" />
    <input type="text" bind:value={back} placeholder="back" />

    <button type="submit">add</button>
</form>

<button on:click={() => history.back()}>Cancel</button>
