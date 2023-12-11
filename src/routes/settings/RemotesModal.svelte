<script lang="ts">
	import { joystick, Direction } from "$lib/joystick";
	import { invoke } from "@tauri-apps/api";
	import Icon from "@iconify/svelte";
	import type { AccessInfo } from "$lib/api";
	import CodeInput from "./CodeInput.svelte";
	import CardModal from "../ui/CardModal.svelte";
	import Button from "../ui/Button.svelte";

    export let device: AccessInfo | undefined;
    $: {
        if (!device) {
            // Try exiting from both components
            joystick.goFrom('remotes/modal:code', Direction.Exit);
            joystick.goFrom('remotes/modal:reject', Direction.Exit);
        }
    }

    const approve = (uuid: string) => {
        console.log(`Approving ${uuid}`);
        invoke('update_pending', { uuid, approve: true });
    };

    const reject = (uuid: string) => {
        console.log(`Rejecting ${uuid}`);
        invoke('update_pending', { uuid, approve: false });
    }
</script>

<CardModal idPrefix='remotes/modal:' center>
    <div id="content">
        <p>Connect <strong>{device?.name}</strong> by typing the code shown on its screen.</p>
        <CodeInput
            code={device?.code ?? '?'}
            component={{
                down: { id: 'remotes/modal:reject' },
                exit: {}
            }}
            onMatch={() => approve(device?.uuid ?? '')}
        />
        <div class="space" />
        <div>
            <Button
                id='remotes/modal:reject'
                onPress={() => device && reject(device.uuid)}
                component={{
                    up: { id: 'remotes/modal:code' },
                    exit: {}
                }}
            >
                <Icon icon='carbon:close' inline /> Reject
            </Button>
        </div>
    </div>
</CardModal>

<style>
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
</style>