<script lang="ts">
    import type { CardSlides } from '@bindings/CardSlides';
    import { fade, fly } from 'svelte/transition';

    export let slides: CardSlides;
    export let currentSlide: number;

    $: visibleSlides = slides.slice(0, currentSlide + 1);
    $: isComplete = currentSlide >= slides.length - 1;
</script>

<button
    class="flex w-full flex-col items-center gap-4 border border-surface-500 p-4 rounded-container-token"
    on:click
>
    {#each visibleSlides as slide, i}
        <div class="prose dark:prose-invert" in:fly={{ y: -10 }}>
            {@html slide}
        </div>

        {#if i < slides.length - 1}
            <hr class="w-full border-surface-500" in:fade />
        {/if}
    {/each}

    {#if !isComplete}
        <i class="text-surface-500" in:fade>Click to Reveal</i>
    {/if}
</button>
