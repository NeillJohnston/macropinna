import { invoke } from "@tauri-apps/api";

export interface DeviceInfo {
    uuid: string;
    name: string;
    code: string;
}

export type RemoteServerEvent = 'RefreshPending';

export const getPendingDevices = async (): Promise<DeviceInfo[]> => {
    return await invoke('get_pending_info_list') as DeviceInfo[];
}