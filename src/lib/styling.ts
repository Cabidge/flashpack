export type ConditionalClass = {
    base?: string;
    on?: string;
    off?: string;
};

export const conditionalClass = (condition: boolean, config: ConditionalClass) => {
    return `${config.base ?? ''} ${condition ? config.on ?? '' : config.off ?? ''}`;
};
