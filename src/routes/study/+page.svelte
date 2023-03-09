<script lang="ts">
    import type { QuizQuery } from '@bindings/QuizQuery';
    import type { PageData } from './$types';

    export let data: PageData;

    $: sessions = data.sessions;

    const sessionId = (id: QuizQuery) => {
        if ('Fleeting' in id) {
            return id.Fleeting;
        } else {
            return id.Concrete;
        }
    };
</script>

<div class="p-8">
    <ul>
        {#each sessions as session (sessionId(session.id))}
            <li>
                {session.name}
            </li>
        {/each}
    </ul>
</div>
