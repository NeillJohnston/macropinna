<script lang="ts">
	import { joystick, nav, Direction } from "$lib/joystick";
	import { onMount } from "svelte";
	import NavBox from "./NavBox.svelte";
	import { invoke } from "@tauri-apps/api";
	import Icon from "@iconify/svelte";
	import type { AccessInfo } from "$lib/api";
	import CodeInput from "./CodeInput.svelte";

    export let device: AccessInfo;

    const approve = (uuid: string) => {
        console.log(`Approving ${uuid}`);
        invoke('update_pending', { uuid, approve: true });
    };

    const reject = (uuid: string) => {
        console.log(`Rejecting ${uuid}`);
        invoke('update_pending', { uuid, approve: false });
    }

    onMount(() => {
        joystick.register('remotes/modal:code', {
            down: {
                id: 'remotes/modal:reject',
            },
            enter: joystick.focusEnter(joystick.focusId('remotes/modal:code')),
            exit: {}
        });

        joystick.register('remotes/modal:reject', {
            up: {
                id: 'remotes/modal:code',
            },
            enter: {
                action: () => device && reject(device.uuid)
            },
            exit: {}
        });

        return () => {
            if ($nav.startsWith('remotes/modal')) {
                joystick.go(Direction.Exit);
            }
        };
    })
</script>

<div id="modal-bg">
    <div id="modal">
        <div id="modal-content">
            <div id="modal-info">
                <p>Connect <strong>{device?.name}</strong> by typing the code shown on its screen.</p>
                <CodeInput
                    code={device?.code ?? '?'}
                    onMatch={() => approve(device?.uuid ?? '')}
                />
            </div>
            <div class="button">
                <NavBox id='remotes/modal:reject'>
                    <p><Icon icon='carbon:close' inline /> Reject</p>
                </NavBox>
            </div>
        </div>
    </div>
</div>

<style>
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

    .button {
        margin: 0.5rem;
    }
    
    .button p {
        text-align: center;
        font-weight: bold;
        margin: 0.5em;
    }
</style>