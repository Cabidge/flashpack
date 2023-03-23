<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let weight: number;

    $: newWeight = "" + weight;

    const dispatch = createEventDispatcher<{ save: number }>();

    const save = () => {
        const weightClean = Math.max(1, parseInt(newWeight ?? ("" + weight)));
        () => dispatch("save", weightClean)
    }
</script>

<input type="number" min=1 bind:value={newWeight} />
<button on:click={save}>save</button>