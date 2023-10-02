<script lang="ts">
	import { Direction, joystick, nav } from "$lib/joystick";
	import { onMount } from "svelte";
    import { listen } from '@tauri-apps/api/event';
	import NavBox from "./NavBox.svelte";
	import { invoke } from "@tauri-apps/api";
	import Icon from "@iconify/svelte";
	import { getPendingDevices, type AccessInfo, type RemoteServerEvent } from "$lib/api";
	import NavLabel from "./NavLabel.svelte";

    let listIndex = 0;
    let list: AccessInfo[] = [];

    const set = (index: number) => {
        listIndex = Math.max(Math.min(index, list.length - 1), 0);
    }

    const approve = (uuid: string) => {
        console.log(`Approving ${uuid}`);
        invoke('update_pending', { uuid, approve: true });
    };

    const reject = (uuid: string) => {
        console.log(`Rejecting ${uuid}`);
        invoke('update_pending', { uuid, approve: false });
    }

    $: device = list.at(listIndex);
    $: showModal = $nav.startsWith('remotes/modal');

    onMount(() => {
        joystick.register('remotes/list', {
            left: {
                action: () => { set(0); }
            },
            up: {
                keep: true,
                action: () => { set(listIndex - 1); }
            },
            down: {
                keep: true,
                action: () => { set(listIndex + 1); }
            },
            enter: {
                id: () => 'remotes/modal:reject',
                keep: true,
                action: () => {
                    if (!device) {
                        joystick.go(Direction.Exit);
                    }
                }
            },
            exit: { alias: Direction.Left }
        });

        joystick.register('remotes/modal:reject', {
            left: {
                id: 'remotes/modal:approve',
            },
            enter: {
                action: () => device && reject(device.uuid)
            },
            exit: {}
        });

        joystick.register('remotes/modal:approve', {
            right: {
                id: 'remotes/modal:reject',
            },
            enter: {
                action: () => device && approve(device.uuid)
            },
            exit: {}
        });

        // TODO listen is an async function (that returns a cleanup function),
        // but we can't return Promises from onMount
        listen('remote_server', async (event) => {
            // TODO switch between event types when needed
            const payload = event.payload as RemoteServerEvent;

            list = await getPendingDevices();
            set(listIndex);
        });

        getPendingDevices().then(_list => {
            list = _list;
        });
    });
</script>

<div id="remotes">
    {#if list.length === 0}
    <NavLabel id='remotes/list' label='(No devices waiting)' />
    {:else}
    {#each list as device, index}
    <NavBox id='remotes/list' and={listIndex === index}>
        <div class='pending'>
            <p><strong>{device.name}</strong></p>
        </div>
    </NavBox>
    {/each}
    {/if}
</div>
<div class:hide={!showModal}>
    <div id="modal-bg">
        <div id="modal">
            <div id="modal-content">
                <div id="modal-info">
                    <p>{device?.name}</p>
                    <p>{device?.code}</p>
                </div>
                <div id="modal-buttons">
                    <div class="button">
                        <NavBox id='remotes/modal:approve'>
                            <p><Icon icon='carbon:checkmark' inline />Approve</p>
                        </NavBox>
                    </div>
                    <div class="button">
                        <NavBox id='remotes/modal:reject'>
                            <p><Icon icon='carbon:close' inline /> Reject</p>
                        </NavBox>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    p {
        margin: 0;
    }

    #remotes {
        width: 100%;
        height: 100%;
        z-index: 0;
    }

    .pending {
        display: flex;
        align-items: center;
        padding: 0.5rem;
    }
    
    .hide {
        display: none;
    }

    #modal-bg {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        z-index: 1;
        background-color: rgba(0, 0, 0, 0.5);
    }

    #modal {
        width: auto;
        height: 100%;
        aspect-ratio: 1/1;
        padding: 6rem;
        box-sizing: border-box;
    }

    #modal-content {
        width: 100%;
        height: 100%;
        border: 1px solid white;
        background-color: black;
        display: flex;
        flex-direction: column;
    }

    #modal-info {
        flex: 1;
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
    }

    #modal-buttons {
        display: flex;
    }

    .button {
        flex: 1;
        margin: 0.5rem;
    }
    
    .button p {
        text-align: center;
        font-weight: bold;
        margin: 0.5em;
    }
</style>