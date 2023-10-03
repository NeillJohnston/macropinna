<script lang="ts">
	import { onMount } from "svelte";
    import { connect, connection } from "$lib/api";
	import Controller from "./Controller.svelte";
    import TextInput from "./ui/TextInput.svelte";
    import Button from "./ui/Button.svelte";

    let name = '';
    let code: string | null = null;
    let error = '';

    // $: connected = $connection !== null;
    $: connected = true;

    const _connect = () => {
        if (name.length === 0) {
            error = 'Please provide a device name';
            return;
        }

        connect(
            { device_name: name },
            (_code: string) => {
                code = _code;
            },
            () => {
                code = null;
            },
            () => {
                code = null;
                error = 'Device rejected';
            },
            () => {
                code = null;
                error = 'Connection closed';
            }
        );
    };
</script>

{#if connected}
<Controller />
{:else}
<div id="root">
    <div id="title">
        <!-- Lmao this is so pretentious -->
        macropinna::remote
    </div>
    {#if !code}
    <div id="connect">
        <div id="name-input">
            <TextInput
                bind:value={name}
                onSubmit={_connect}
                placeholder="Device name"
            />
        </div>
        {#if error}
        <div id="error">{error}</div>
        {/if}
        <Button onClick={_connect}><span class="no-select">Connect</span></Button>
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

    #title {
        position: fixed;
        top: 0.5rem;
        font-weight: bold;
        font-size: 0.71rem;
        letter-spacing: 0.1em;
    }

    #connect {
        text-align: center;
    }

    #name-input {
        margin-bottom: 0.5rem;
    }

    #error {
        color: var(--err);
        font-size: 0.71rem;
        margin-bottom: 0.5rem;
    }

    #code {
        font-size: 2.00rem;
        font-weight: bold;
        letter-spacing: 0.25rem;
    }
</style>