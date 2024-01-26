import { invoke } from "@tauri-apps/api";
import { listen, type Event } from "@tauri-apps/api/event";
import { get, writable } from "svelte/store";

export interface Launcher {
    name: string;
    command: string;
    finder: string;
    finder_is_regex?: boolean;
    image_path?: string;
    css_background?: string;
};

export interface Config {
    name: string;
    theme: {
        color: string;
        style: string;
    };
    home: {
        screens: {
            widgets: {
                name: string;
                coords: {
                    x: number;
                    y: number;
                    w: number;
                    h: number;
                };
                props?: any;
            }[];
        }[];
    };
    launchers: Launcher[];
    weather?: {
        provider: 'OpenWeatherMap',
        api_key: string;
        lat: number;
        long: number;
    };
    audio_device?: {
        name: string;
    };
    remote_server: {
        port: number;
    };
    needs_setup?: boolean;
}

interface ConfigEvent {
    Set: {
        config: Config;
    };
}

// Providing a minimal config for typing purposes
export const config = writable<Config>({
    name: '',
    theme: {
        color: '',
        style: ''
    },
    launchers: [],
    home: {
        screens: []
    },
    remote_server: {
        port: 0
    },
});

export const listenConfig = async () => {
    const init = await invoke('get_config') as Config;
    config.set(init);

    return listen('config', (event: Event<ConfigEvent>) => {
        config.set(event.payload.Set.config);
    });
}

export const setConfig = async (newConfig: Config) => {
    await invoke('set_config', { newConfig });
}

export type Agent = 'Android' | 'IPhone' | 'Desktop' | 'Unknown';

export interface AccessInfo {
    uuid: string;
    name: string;
    agent: Agent;
    code: string;
}

export interface ActiveInfo {
    uuid: string;
    name: string;
    agent: Agent;
}

export const getPendingDevices = async (): Promise<AccessInfo[]> => {
    return await invoke('get_pending_info_list') as AccessInfo[];
}

export const getActiveDevices = async (): Promise<ActiveInfo[]> => {
    return await invoke('get_active_info_list') as ActiveInfo[];
}

export const getRemoteServerIp = async (): Promise<string | undefined> => {
    return await invoke('get_remote_server_ip') as (string | undefined);
}