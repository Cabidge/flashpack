import type { QuizQuery } from '@bindings/QuizQuery';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params }) => {
    const isFleeting = params.fleeting === 'fleeting';

    const id: QuizQuery = isFleeting
        ? { Fleeting: parseInt(params.id, 10) }
        : { Concrete: params.id };

    return { id };
};
