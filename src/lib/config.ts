// import { invoke } from '@tauri-apps/api';
import { writable } from 'svelte/store';

interface Widget {
    x: number;
    y: number;
    w: number;
    h: number;
}

type XAlign = 'left' | 'center' | 'right';
type YAlign = 'top' | 'middle' | 'bottom';

interface Config {
    name: string;
    home: {
        clock: {
            enabled: boolean,
            widget: Widget,
            xAlign: XAlign,
            yAlign: YAlign,
        },
        weather: {
            enabled: boolean,
            widget: Widget,
            xAlign: XAlign,
            yAlign: YAlign,
        },
        todo: {
            enabled: boolean,
            widget: Widget,
            xAlign: XAlign,
        },
    }
}

export const config = writable({
    name: 'Neill',
    home: {
        clock: {
            enabled: true,
            widget: { x: 0, y: 0, w: 6, h: 3 },
            xAlign: 'right',
            yAlign: 'middle'
        },
        weather: {
            enabled: true,
            widget: { x: 6, y: 0, w: 6, h: 3 },
            xAlign: 'left',
            yAlign: 'middle'
        },
        todo: {
            enabled: true,
            widget: { x: 6, y: 3, w: 6, h: 9 },
            xAlign: 'left',
        },
    }
} as Config);

export async function loadConfig(): Promise<void> {
    // const _config = await invoke('load_config') as Config;
    // config.set(_config);
}

export function saveConfig() {
    // config.subscribe(config => {
    //     invoke('save_config', { config });
    // })
}