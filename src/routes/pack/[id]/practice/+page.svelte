<script lang="ts">
    import { goto } from '$app/navigation';
    import { AppBar, Step, Stepper } from '@skeletonlabs/skeleton';
    import { fade, fly } from 'svelte/transition';

    export let data;

    $: questions = data.questions.map((slides) => ({ slides, current: 0 }));
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
            {@const visibleSlides = question.slides.slice(0, question.current + 1)}
            {@const isComplete = question.current >= question.slides.length - 1}

            <Step locked={!isComplete} regionContent="flex justify-center">
                <button
                    class="flex w-full flex-col items-center gap-4 border border-surface-500 p-4 rounded-container-token"
                    on:click={() => {
                        if (!isComplete) {
                            question.current += 1;
                        }
                    }}
                >
                    {#each visibleSlides as slide, i}
                        <div class="prose dark:prose-invert" in:fly={{ y: -10 }}>
                            {@html slide}
                        </div>

                        {#if i < question.slides.length - 1}
                            <hr class="w-full border-surface-500" in:fade />
                        {/if}
                    {/each}

                    {#if !isComplete}
                        <i class="text-surface-500" in:fade>Click to Reveal</i>
                    {/if}
                </button>
            </Step>
        {/each}
    </Stepper>
</div>
