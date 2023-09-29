<script lang="ts">
	import { onMount } from "svelte";
    import { connect, connection } from "$lib/api";
	import Controller from "./Controller.svelte";

    console.log('hello');

    let name = '';
    let code: string | null = null;

    $: connected = $connection !== null;

    const _connect = () => {
        connect(
            { device_name: name },
            (_code: string) => {
                code = _code;
            },
            () => {
                console.log('Connected');
                code = null;
            },
            () => {
                console.log('Rejected');
                code = null;
            },
            () => {
                console.log('Closed');
                code = null;
            }
        );
    };
</script>

{#if connected}
<Controller />
{:else}
<div id="root">
    {#if !code}
    <div id="name">
        <input type="text" bind:value={name} />
        <button on:click={_connect}>Go</button>
    </div>
    {:else}
    <div id="code" class="mono">
        {code}
    </div>
    {/if}
</div>
{/if}

<style>
    #root {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    #name {}

    #code {
        font-size: 2.00rem;
    }
</style>