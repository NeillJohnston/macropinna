<script lang="ts">
	import { Direction, joystick, nav } from "$lib/joystick";
	import { onMount } from "svelte";
    import { listen } from '@tauri-apps/api/event';
	import NavBox from "./NavBox.svelte";
	import { invoke } from "@tauri-apps/api";
	import Icon from "@iconify/svelte";
	import { getPendingDevices, type AccessInfo, type RemoteServerEvent } from "$lib/api";
	import NavLabel from "./NavLabel.svelte";
	import CodeInput from "./CodeInput.svelte";
	import RemotesModal from "./RemotesModal.svelte";

    let listIndex = 0;
    let list: AccessInfo[] = [];

    const set = (index: number) => {
        listIndex = Math.max(Math.min(index, list.length - 1), 0);
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
                id: 'remotes/modal:code',
                keep: true,
                action: () => {
                    if (!device) {
                        joystick.go(Direction.Exit);
                    }
                }
            },
            exit: { alias: Direction.Left }
        });

        // TODO listen is an async function (that returns a cleanup function),
        // but we can't return Promises from onMount
        listen('remote_server', async (event) => {
            // TODO switch between event types when needed
            const payload = event.payload as RemoteServerEvent;

            list = await getPendingDevices();
            set(listIndex);

            // Clean up modal if necessary
            if (!list.find(dev => dev.uuid === device?.uuid)) {
                device = undefined;
                // joystick.goFrom('remotes/modal:code', Direction.Exit);
                // joystick.goFrom('remotes/modal:reject', Direction.Exit);
            }
        });

        getPendingDevices().then(_list => {
            list = _list;
        });
    });
</script>

<div id="remotes">
    {#if list.length === 0}
    <NavLabel id='remotes/list'>(No devices waiting)</NavLabel>
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
{#if showModal && device}
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

    .pending {
        display: flex;
        align-items: center;
        padding: 0.5rem;
    }
</style>