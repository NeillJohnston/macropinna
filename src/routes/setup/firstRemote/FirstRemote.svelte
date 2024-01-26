<script lang="ts">
	import { onMount } from "svelte";
	import type { ScreenProps } from "../+page.svelte";
	import ContinueButton from "../ContinueButton.svelte";
	import { Direction, joystick } from "$lib/joystick";
	import QRCode from "@castlenine/svelte-qrcode";
	import { getActiveDevices, getPendingDevices, getRemoteServerIps, type AccessInfo, type ActiveInfo, type RemoteServerIp } from "$lib/api";
	import MenuSection from "../../ui/MenuSection.svelte";
	import CardModal from "../../ui/CardModal.svelte";
	import CodeInput from "./CodeInput.svelte";
	import Button from "../../ui/Button.svelte";
	import { invoke } from "@tauri-apps/api";
	import Icon from "@iconify/svelte";
	import DeviceName from "../../ui/DeviceName.svelte";
	import CarouselSelector from "../../ui/CarouselSelector.svelte";

    export let props: ScreenProps;

    const id = 'first-remote';
    const continueButtonId = 'first-remote/continue';
    
    let error = false;

    const refresh = () => {
        try {
            getPendingDevices().then(pendingList => {
                const existingDevice = !!device;
                device = pendingList[0];
                if (!existingDevice && !!device) {
                    joystick.push('first-remote/modal:code');
                }
                else if (existingDevice && !device) {
                    joystick.goFrom('first-remote/modal:code', Direction.Exit);
                    joystick.goFrom('first-remote/modal:reject', Direction.Exit);
                }
            });

            getActiveDevices().then(list => {
                activeList = list;
            });

            error = false;
        }
        catch (_error) {
            error = true;
        }
    };

    let ips: RemoteServerIp[] = [];
    let ipNames: string[] = [];
    let index = 0;
    let device: AccessInfo | undefined = undefined;
    let activeList: ActiveInfo[] = [];
    onMount(() => {
        getActiveDevices().then(_activeList => {
            activeList = _activeList;
        })

        getRemoteServerIps().then(_ips => {
            ips = _ips;
            ipNames = ips.map(ip => ip.name);
        });

        refresh();
        const refreshInterval = setInterval(refresh, 500);
        return () => {
            clearInterval(refreshInterval);
        }
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
        <div id="qr-inner">
            {#if ips.length > 0}
            <div class="overlay" id="carousel">
                <CarouselSelector
                    id={id}
                    bind:index={index}
                    values={ipNames}
                    component={{
                        up: { action: props.prev },
                        down: { id: continueButtonId }
                    }}
                />
            </div>
            <div class="overlay" id="address">
                {ips[index].ip}
            </div>
            {#each ips as ip, _index}
            <div class:hidden={index !== _index}>
                <QRCode content={ip.ip} padding={2} isResponsive />
            </div>
            {/each}
            {:else}
            <p><strong>Could not find a suitable IP address for the remote server.</strong></p>
            <p>Check your console or log for errors.</p>
            {/if}
        </div>
    </div>
    <div id="connected-list">
        {#if activeList.length === 0}
        <p class="small"><em>Macropinna is best with a remote - scan the QR code to add your device!</em></p>
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

    #qr-inner {
        width: 100%;
        height: 100%;
        position: relative;
    }

    .overlay {
        width: 100%;
        position: absolute;
        color: black;
        font-size: var(--f-1);
    }

    #carousel {
        top: 0;
    }

    #address {
        text-align: center;
        bottom: 0;
    }

    .hidden {
        display: none;
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