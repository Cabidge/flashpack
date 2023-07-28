<script lang="ts">
    import { invoke } from '$lib/commands.js';
    import InteractiveSlides from '$lib/components/InteractiveSlides.svelte';
    import type { CardSlides } from '@bindings/CardSlides.js';
    import { AppBar } from '@skeletonlabs/skeleton';
    import { onMount } from 'svelte';

    export let data;

    let slides: Promise<CardSlides> = new Promise(() => {});
    let currentSlide: number;

    const next = () => {
        const cardIndex = Math.floor(Math.random() * data.cards.length);
        const card = data.cards[cardIndex];

        slides = invoke('generate_card_slides', { script: card.script, template: card.template });
        currentSlide = 0;
    };

    onMount(next);
</script>

<AppBar>
    <svelte:fragment slot="lead">
        <a href="/pack/{data.pack.id}">
            <i class="fa-solid fa-arrow-left fa-lg" />
        </a>
    </svelte:fragment>

    <h1 class="text-xl font-semibold">Endless Practice</h1>
</AppBar>

<div class="space-y-4 p-4">
    {#await slides then slides}
        <InteractiveSlides
            {slides}
            {currentSlide}
            on:click={() => {
                if (currentSlide >= slides.length - 1) {
                    next();
                } else {
                    currentSlide += 1;
                }
            }}
        />
    {/await}
</div>
