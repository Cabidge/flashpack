import { writable } from 'svelte/store';

export type Tab = 'pack' | 'study';

export const activeTab = writable<Tab>('pack');
