<script lang="ts">
    import { studies } from '$lib/stores/studies';
</script>

<button>Add Study</button>

<ul>
    {#each $studies as study (study.id)}
        <li>
            <a href="/study/edit/{study.id}">{study.title}</a>
            <!-- TODO: implement this -->
            <button disabled>Begin Practice</button>
        </li>
    {/each}
</ul>
