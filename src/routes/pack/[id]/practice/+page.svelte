<script lang="ts">
    import { goto } from '$app/navigation';
    import InteractiveSlides from '$lib/components/InteractiveSlides.svelte';
    import { AppBar, Step, Stepper } from '@skeletonlabs/skeleton';

    export let data;

    $: questions = data.questions.map((slides) => ({ slides, currentSlide: 0 }));
</script>

<AppBar>
    <svelte:fragment slot="lead">
        <a href="/pack/{data.id}">
            <i class="fa-solid fa-arrow-left fa-lg" />
        </a>
    </svelte:fragment>

    <h1 class="text-xl font-semibold">Practice</h1>
</AppBar>

<div class="p-4">
    <Stepper on:complete={() => goto(`/pack/${data.id}`)} stepTerm="Question">
        {#each questions as question}
            <Step
                locked={question.currentSlide < question.slides.length - 1}
                regionContent="flex justify-center"
            >
                <InteractiveSlides {...question} on:click={() => (question.currentSlide += 1)} />
            </Step>
        {/each}
    </Stepper>
</div>
