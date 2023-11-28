<script lang="ts">
	import { onMount } from "svelte";
	import type { ScreenProps } from "../+page.svelte";
	import ContinueButton from "../ContinueButton.svelte";
	import { Direction, joystick, nav } from "$lib/joystick";
	import QRCode from "@castlenine/svelte-qrcode";
	import { getActiveDevices, getPendingDevices, getRemoteServerIp, type AccessInfo, type RemoteServerEvent, type ActiveInfo } from "$lib/api";
	import MenuSection from "../../ui/MenuSection.svelte";
	import { listen } from "@tauri-apps/api/event";
	import CardModal from "../../ui/CardModal.svelte";
	import CodeInput from "./CodeInput.svelte";
	import Button from "../../ui/Button.svelte";
	import { invoke } from "@tauri-apps/api";
	import Icon from "@iconify/svelte";
	import DeviceName from "../../ui/DeviceName.svelte";

    export let props: ScreenProps;

    const id = 'first-remote';
    const continueButtonId = 'first-remote/continue';

    let ip: string | undefined = undefined;
    let device: AccessInfo | undefined = undefined;
    let activeList: ActiveInfo[] = [];
    onMount(() => {
        joystick.register(id, {
            up: { action: props.prev },
            down: { id: continueButtonId }
        });

        listen('remote_server', async (event) => {
            const payload = event.payload as RemoteServerEvent;

            if (payload === 'RefreshPending') {
                const pendingList = await getPendingDevices();
                
                const existingDevice = !!device;
                device = pendingList[0];
                if (!existingDevice && !!device) {
                    joystick.push('first-remote/modal:code');
                }
                else if (existingDevice && !device) {
                    joystick.goFrom('first-remote/modal:code', Direction.Exit);
                    joystick.goFrom('first-remote/modal:reject', Direction.Exit);
                }
            }
            else {
                activeList = await getActiveDevices();
            }
        });

        getActiveDevices().then(_activeList => {
            activeList = _activeList;
        })

        getRemoteServerIp().then(_ip => {
            ip = _ip;
        });
    });

    const approve = (uuid: string) => {
        console.log(`Approving ${uuid}`);
        invoke('update_pending', { uuid, approve: true });
    };

    const reject = (uuid: string) => {
        console.log(`Rejecting ${uuid}`);
        invoke('update_pending', { uuid, approve: false });
    };
</script>

<div id="first-remote">
    <div id="qr">
        {#if ip}
        <QRCode content={ip} padding={2} isResponsive />
        {/if}
    </div>
    <div id="connected-list">
        {#if activeList.length === 0}
        <p class="small"><em>No connected devices - scan the QR code to start adding your device as a remote!</em></p>
        {:else}
        <MenuSection label='Connected devices'>
            {#each activeList as active}
            <p><DeviceName name={active.name} agent={active.agent} /></p>
            {/each}
        </MenuSection>
        {/if}
    </div>
</div>
<ContinueButton
    id={continueButtonId}
    component={{
        up: { id: id }
    }}
    onPress={props.next}
/>
<CardModal idPrefix='first-remote/modal:' center>
    <div id="content">
        {#if device}
        <p>Connect <strong><DeviceName name={device.name} agent={device.agent} /></strong> by typing the code shown on its screen.</p>
        {/if}
        <CodeInput
            code={device?.code ?? '?'}
            component={{
                down: { id: 'first-remote/modal:reject' },
                exit: {}
            }}
            onMatch={() => approve(device?.uuid ?? '')}
        />
        <div class="space" />
        <div>
            <Button
                id='first-remote/modal:reject'
                onPress={() => device && reject(device.uuid)}
                component={{
                    up: { id: 'first-remote/modal:code' },
                    exit: {}
                }}
            >
                <Icon icon='carbon:close' inline /> Reject
            </Button>
        </div>
    </div>
</CardModal>

<style>
    #first-remote {
        width: 100%;
        height: 100%;
        display: flex;
    }

    #qr {
        width: auto;
        height: 100%;
        aspect-ratio: 1/1;
        box-sizing: border-box;
        padding: var(--lg);
    }

    #connected-list {
        margin-left: var(--md);
    }

    #content {
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
        padding: var(--md);
    }

    .space {
        height: var(--lg);
    }

    .small {
        font-size: var(--f-1);
    }
</style>