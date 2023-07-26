<script lang="ts">
    import { invoke } from '$lib/commands';

    export let script: string = '';
    export let template: string;

    $: rendered = invoke('generate_card_slides', { script, template }).then((slides) =>
        slides.join('<hr>')
    );
</script>

<div class="prose card w-modal p-4 shadow-xl dark:prose-invert prose-hr:my-4">
    {#await rendered then rendered}
        {@html rendered}
    {/await}
</div>
