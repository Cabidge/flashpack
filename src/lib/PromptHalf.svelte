<script lang="ts">
    import { onMount } from 'svelte';
    import katex from 'katex';

    export let input: string;

    let root: HTMLDivElement;

    onMount(() => {
        let mathElements = [...root.getElementsByClassName('language-math')];

        for (const element of mathElements) {
            const displayMode = element.classList.contains('math-display');

            const unescape = (text: string) => {
                const txt = document.createElement('textarea');
                txt.innerHTML = text;
                return txt.value;
            };

            const tex = unescape(element.innerHTML);

            const texHtml = katex.renderToString(tex, {
                displayMode,
                throwOnError: false,
                output: 'mathml'
            });

            const texElement = new DOMParser().parseFromString(texHtml, 'text/html').body
                .children[0];

            element.parentElement?.replaceChild(texElement, element);
        }
    });
</script>

<div bind:this={root} class="prose">
    {@html input}
</div>
