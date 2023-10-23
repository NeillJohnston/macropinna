<script lang="ts">
	import { onMount } from "svelte";
	import Modal from "../ui/Modal.svelte";
    import QRCode from '@castlenine/svelte-qrcode';
	import { Direction, joystick } from "$lib/joystick";

    export let ip: string | undefined;

    onMount(() => {
        joystick.register('remotes/qr', {
            enter: {},
            exit: { alias: Direction.Enter }
        });
    })
</script>

<Modal>
    <div id="modal">
        {#if ip}
        <div id="modal-content">
            <QRCode content={ip} padding={2} isResponsive />
            <!-- <p id="info"><em>Scan to connect your device.</em></p> -->
        </div>
        {:else}
        <div id="modal-content-err">
            <p id="info">
                <strong><em>Could not find a suitable IP address for the remote server.</em></strong><br />
                <em>Check your console or log for errors.</em>
            </p>
        </div>
        {/if}
    </div>
</Modal>

<style>
    #modal {
        width: auto;
        height: 100%;
        aspect-ratio: 1/1;
        padding: 4rem;
        box-sizing: border-box;
        text-align: center;
    }

    #modal-content {
        width: 100%;
        height: 100%;
    }

    #modal-content-err {
        width: 100%;
        height: 100%;
        border: 1px solid var(--fg);
        background-color: var(--bg);
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    #info {
        font-size: 0.71rem;
    }
</style>