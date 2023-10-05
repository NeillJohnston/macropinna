import { invoke } from "@tauri-apps/api";
import { listen, type Event } from "@tauri-apps/api/event";
import { writable } from "svelte/store";
import type { Layout } from "./layout";

export interface Config {
    name: string;
    home: Layout;
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
}

interface ConfigEvent {
    Set: {
        config: Config;
    };
}

// Providing a minimal config for typing purposes
export const config = writable<Config>({
    name: '',
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
    console.log(init);

    return listen('config', (event: Event<ConfigEvent>) => {
        console.log(event.payload.Set.config);
        config.set(event.payload.Set.config);
    });
}

export interface AccessInfo {
    uuid: string;
    name: string;
    code: string;
}

export interface ActiveInfo {
    uuid: string;
    name: string;
}

export type RemoteServerEvent = 'RefreshPending' | 'RefreshActive' | {
    Connected: {
        name: string;
        uuid: string;
    }
};

export const getPendingDevices = async (): Promise<AccessInfo[]> => {
    return await invoke('get_pending_info_list') as AccessInfo[];
}

export const getActiveDevices = async (): Promise<ActiveInfo[]> => {
    return await invoke('get_active_info_list') as ActiveInfo[];
}

export const getRemoteServerIp = async (): Promise<string | undefined> => {
    return await invoke('get_remote_server_ip') as (string | undefined);
}