<script lang="ts">
    import { modals } from '$lib/modals';
    import { studies } from '$lib/stores/studies';
    import CreateStudy from './CreateStudy.svelte';
</script>

<button on:click={() => modals.add(CreateStudy, {})}>Add Study</button>

<ul>
    {#each $studies as study (study.id)}
        <li>
            <a href="/study/edit/{study.id}">{study.title}</a>
            <!-- TODO: implement this -->
            <button disabled>Begin Practice</button>
        </li>
    {/each}
</ul>
