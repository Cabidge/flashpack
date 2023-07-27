<script lang="ts">
    import { goto } from '$app/navigation';
    import { AppBar, Step, Stepper } from '@skeletonlabs/skeleton';

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
            {@const isEnd = question.current >= question.slides.length - 1}
            <Step locked={!isEnd} regionContent="flex justify-center">
                <button
                    class="prose w-full border border-surface-500 p-4 rounded-container-token dark:prose-invert prose-hr:my-4"
                    on:click={() => {
                        if (!isEnd) {
                            question.current += 1;
                        }
                    }}
                >
                    {@html visibleSlides.join('<hr>')}
                    {#if !isEnd}
                        <hr />
                        <i>Click to Reveal</i>
                    {/if}
                </button>
            </Step>
        {/each}
    </Stepper>
</div>
