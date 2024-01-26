<script lang="ts">
	import { Direction, joystick, nav } from "$lib/joystick";
	import { onMount } from "svelte";
    import { listen } from '@tauri-apps/api/event';
	import NavBox from "../ui/NavBox.svelte";
	import { getPendingDevices, type AccessInfo, getRemoteServerIp, type ActiveInfo, getActiveDevices } from "$lib/api";
	import NavLabel from "../ui/NavLabel.svelte";
	import RemotesModal from "./RemotesModal.svelte";
	import QrModal from "./QrModal.svelte";
	import MenuSection from "../ui/MenuSection.svelte";
	import Button from "../ui/Button.svelte";
	import Icon from "@iconify/svelte";
	import DeviceName from "../ui/DeviceName.svelte";

    let pendingList: AccessInfo[] = [];
    let activeList: ActiveInfo[] = [];

    let ip: string | undefined = undefined;
    let device: AccessInfo | undefined;

    $: showQrModal = $nav === 'remotes/qr';

    // Build (or rebuild) all of the nav components - each list element gets its
    // own component
    const buildNav = () => {
        const end = (list: any[]): number => Math.max(list.length - 1, 0);

        if (pendingList.length === 0) {
            joystick.register('remotes/pending/0', {
                left: {},
                exit: { alias: Direction.Left },
                up: { id: 'remotes/showqr' },
                down: { id: 'remotes/active/0' }
            });
        }
        else {
            for (const [i, _device] of pendingList.entries()) {
                const prevId = i === 0 ? 'remotes/showqr' : `remotes/pending/${i-1}`;
                const nextId = i === end(pendingList) ? 'remotes/active/0' : `remotes/pending/${i+1}`;

                joystick.register(`remotes/pending/${i}`, {
                    left: {},
                    exit: { alias: Direction.Left },
                    enter: {
                        keep: true,
                        id: 'remotes/modal:code',
                        action: () => {
                            device = _device;
                        }
                    },
                    up: { id: prevId },
                    down: { id: nextId },
                });
            }
        }

        if (activeList.length === 0) {
            joystick.register('remotes/active/0', {
                left: {},
                exit: { alias: Direction.Left },
                up: { id: `remotes/pending/${end(pendingList)}` },
            });
        }
        else {
            for (const [i, _device] of activeList.entries()) {
                const prevId = i === 0 ? `remotes/pending/${end(pendingList)}` : `remotes/active/${i-1}`;
                const nextId = `remotes/active/${i+1}`;

                joystick.register(`remotes/active/${i}`, {
                    left: {},
                    exit: { alias: Direction.Left },
                    up: { id: prevId },
                    down: i === end(activeList) ? undefined : { id: nextId },
                });
            }
        }
    }
    
    const refresh = () => {
        getPendingDevices().then(list => {
            pendingList = list;
            buildNav();
        });

        getActiveDevices().then(list => {
            activeList = list;
            buildNav();
        });
    }

    onMount(() => {
        getRemoteServerIp().then(_ip => {
            ip = _ip;
        });

        refresh();
        const refreshInterval = setInterval(refresh, 1000);
        return () => {
            clearInterval(refreshInterval);
        }
    });
</script>

<div id="remotes">
    <MenuSection>
        <div class='item'>
            <Button
                id='remotes/showqr'
                onPress={() => joystick.push('remotes/qr')}
                component={{
                    left: {},
                    exit: { alias: Direction.Left },
                    down: { id: 'remotes/pending/0' },
                }}
            >
                <Icon icon='carbon:qr-code' inline /> <strong>Show QR code</strong>
            </Button>
        </div>
    </MenuSection>
    <MenuSection label='Pending connections'>
        {#if pendingList.length === 0}
        <div class='item'>
            <NavLabel id='remotes/pending/0'>(No devices waiting)</NavLabel>
        </div>
        {:else}
        {#each pendingList as device, index}
        <NavBox id={`remotes/pending/${index}`}>
            <div class='item'>
                <strong><DeviceName name={device.name} agent={device.agent} /></strong>
            </div>
        </NavBox>
        {/each}
        {/if}
    </MenuSection>
    <MenuSection label='Active connections'>
        {#if activeList.length === 0}
        <div class='item'>
            <NavLabel id='remotes/active/0'>(No devices active)</NavLabel>
        </div>
        {:else}
        {#each activeList as device, index}
        <NavBox id={`remotes/active/${index}`}>
            <div class='item'>
                <strong><DeviceName name={device.name} agent={device.agent} /></strong>
            </div>
        </NavBox>
        {/each}
        {/if}
    </MenuSection>
</div>
{#if showQrModal}
<QrModal ip={ip} />
{/if}
<RemotesModal device={device} />

<style>
    #remotes {
        width: 100%;
        height: 100%;
        z-index: 0;
    }

    .item {
        display: inline-block;
        padding: var(--sm);
    }
</style>