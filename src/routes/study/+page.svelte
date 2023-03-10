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

    const sessionLink = (id: QuizQuery) =>
        'Fleeting' in id ? `/study/fleeting/${id.Fleeting}` : `/study/concrete/${id.Concrete}`;
</script>

<div class="p-8">
    <ul>
        {#each sessions as session (sessionId(session.id))}
            <li>
                <a href={sessionLink(session.id)}>{session.name}</a>
            </li>
        {/each}
    </ul>
</div>
