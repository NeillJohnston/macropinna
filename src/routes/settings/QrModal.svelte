<script lang="ts">
	import { onMount } from "svelte";
    import QRCode from '@castlenine/svelte-qrcode';
	import { Direction, joystick } from "$lib/joystick";
	import CardModal from "../ui/CardModal.svelte";

    export let ip: string | undefined;

    onMount(() => {
        joystick.register('remotes/qr', {
            exit: {},
            enter: { alias: Direction.Exit }
        });
    })
</script>

<CardModal idPrefix='remotes/qr' center aspectRatio='1/1'>
    {#if ip}
    <div id="qr">
        <QRCode content={ip} padding={2} isResponsive />
    </div>
    {:else}
    <div id="err">
        <p><strong>Could not find a suitable IP address for the remote server.</strong></p>
        <p>Check your console or log for errors.</p>
    </div>
    {/if}
</CardModal>

<style>
    #qr {
        width: 100%;
        height: 100%;
    }

    #err {
        width: 100%;
        font-size: var(--f-1);
        text-align: center;
    }
</style>