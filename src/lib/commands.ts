import type { invoke as invoke_ } from '@tauri-apps/api';

type Window = {
    __TAURI__: {
        invoke: typeof invoke_;
    };
};

export const invoke: typeof invoke_ = async (cmd, args) => {
    const invoke = (window as unknown as Window).__TAURI__.invoke;
    return await invoke(cmd, args);
};
