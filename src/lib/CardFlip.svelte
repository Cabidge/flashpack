<script lang="ts">
    export let front: string;
    export let back: string;

    export let flipped = false;

    $: displayed = flipped ? back : front;
</script>

<button class="card" class:flipped on:click={() => (flipped = !flipped)}>
    <h2>{displayed}</h2>
</button>

<style>
    .card {
        all: unset;

        border: 1px solid black;
        background: white;
        text-align: center;

        width: 80vw;

        cursor: pointer;

        transition: all 0.3s ease-out;
    }

    .card.flipped {
        transform: rotateX(360deg);
        background: rgb(230, 230, 230);
    }
</style>
