<script lang="ts">
	import { Direction, joystick, nav } from "$lib/joystick";
	import { onMount } from "svelte";
    import { listen } from '@tauri-apps/api/event';
	import NavBox from "../ui/NavBox.svelte";
	import { getPendingDevices, type AccessInfo, type RemoteServerEvent, getRemoteServerIp, type ActiveInfo, getActiveDevices } from "$lib/api";
	import NavLabel from "../ui/NavLabel.svelte";
	import RemotesModal from "./RemotesModal.svelte";
	import QrModal from "./QrModal.svelte";

    let pendingList: AccessInfo[] = [];
    let activeList: ActiveInfo[] = [];

    let ip: string | undefined = undefined;
    let device: AccessInfo | undefined;

    $: showRemotesModal = !!device && $nav.startsWith('remotes/modal');
    $: showQrModal = $nav === 'remotes/qr';

    // Build (or rebuild) all of the nav components - each list element gets its
    // own component
    const buildNav = () => {
        const end = (list: any[]): number => Math.max(list.length - 1, 0);

        joystick.register('remotes/showqr', {
            left: {},
            exit: { alias: Direction.Left },
            enter: {
                keep: true,
                id: 'remotes/qr'
            },
            down: { id: 'remotes/pending/0' },
        });

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

    onMount(() => {
        // TODO listen is an async function (that returns a cleanup function),
        // but we can't return Promises from onMount
        listen('remote_server', async (event) => {
            // TODO switch between event types when needed
            const payload = event.payload as RemoteServerEvent;
            console.log(payload);

            if (payload === 'RefreshPending') {
                pendingList = await getPendingDevices();
            }
            else {
                activeList = await getActiveDevices();
            }

            buildNav();

            // Clean up modal if necessary
            if (!pendingList.find(dev => dev.uuid === device?.uuid)) {
                device = undefined;
            }
        });

        getPendingDevices().then(list => {
            pendingList = list;
            buildNav();
        });

        getActiveDevices().then(list => {
            activeList = list;
            buildNav();
        });

        getRemoteServerIp().then(_ip => {
            ip = _ip;
        });
    });
</script>

<div id="remotes">
    <div class='item'>
        <NavLabel id='remotes/showqr'><strong>Show QR code</strong></NavLabel>
    </div>
    <div class="label">Pending connections</div>
    {#if pendingList.length === 0}
    <div class='item'>
        <NavLabel id='remotes/pending/0'>(No devices waiting)</NavLabel>
    </div>
    {:else}
    {#each pendingList as device, index}
    <NavBox id={`remotes/pending/${index}`}>
        <div class='item'>
            <p><strong>{device.name}</strong></p>
        </div>
    </NavBox>
    {/each}
    {/if}
    <div class="label">Active connections</div>
    {#if activeList.length === 0}
    <div class='item'>
        <NavLabel id='remotes/active/0'>(No devices active)</NavLabel>
    </div>
    {:else}
    {#each activeList as device, index}
    <NavBox id={`remotes/active/${index}`}>
        <div class='item'>
            <p><strong>{device.name}</strong></p>
        </div>
    </NavBox>
    {/each}
    {/if}
</div>
{#if showQrModal}
<QrModal ip={ip} />
{/if}
{#if showRemotesModal && device}
<RemotesModal device={device} />
{/if}

<style>
    p {
        margin: 0;
    }

    #remotes {
        width: 100%;
        height: 100%;
        z-index: 0;
    }

    .label {
        margin-top: 1em;
        margin-bottom: 1em;
        font-size: 0.71rem;
    }

    .item {
        display: flex;
        align-items: center;
        padding: 0.5rem;
    }
</style>