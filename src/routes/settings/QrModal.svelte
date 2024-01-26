<script lang="ts">
    import QRCode from '@castlenine/svelte-qrcode';
	import { Direction } from "$lib/joystick";
	import CardModal from "../ui/CardModal.svelte";
	import type { RemoteServerIp } from "$lib/api";
	import CarouselSelector from "../ui/CarouselSelector.svelte";

    export let ips: RemoteServerIp[];
    const ipNames = ips.map(ip => ip.name);

    let index = 0;
</script>

<CardModal idPrefix='remotes/qr' center aspectRatio='1/1'>
    {#if ips.length > 0}
    <div id="qr">
        <div class="overlay" id="carousel">
            <CarouselSelector
                id={'remotes/qr'}
                bind:index={index}
                values={ipNames}
                component={{
                    exit: {},
                    enter: { alias: Direction.Exit }
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

    #err {
        width: 100%;
        font-size: var(--f-1);
        text-align: center;
    }
</style>