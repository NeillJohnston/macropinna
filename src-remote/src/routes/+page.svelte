<script lang="ts">
	import { onMount } from "svelte";
    import { connect, connectWs, connection } from "$lib/api";
	import Controller from "./Controller.svelte";
    import TextInput from "./ui/TextInput.svelte";
    import Button from "./ui/Button.svelte";

    let name = '';
    let code: string | null = null;
    let error = '';
    let reconnect: 'allow' | 'deny' | 'pending' = 'allow';

    $: connected = $connection !== null;
    // $: connected = true;

    // Try to reconnect from a stored JWT
    const tryReconnect = () => {
        // TODO conditions for removing this from storage? Right now onClose is
        // called whether the connection gets dropped or never established in
        // the first place due to bad auth (which will happen once expiration is
        // added to the auth API)
        const jwt = localStorage.getItem('auth-token');
        if (jwt) {
            reconnect = 'pending';
            connectWs(jwt, onAccept, onClose);
        }
    }

    const onInit = (_code: string) => {
        code = _code;
    };

    const onJwt = (jwt: string) => {
        localStorage.setItem('auth-token', jwt);
    };
    
    const onAccept = () => {
        code = null;
        reconnect = 'allow';
    };

    const onReject = () => {
        code = null;
        error = 'Device rejected';
    };

    const onClose = () => {
        // Connections may be closed on our side due to the screen being off -
        // setTimeout with no timeout param forces this code to run on the next
        // render cycle, a.k.a. the next time the screen is turned on again
        code = null;
        error = 'Connection closed';

        if (reconnect === 'allow') {
            setTimeout(() => {
                tryReconnect();
            });
        }
        else {
            reconnect = 'deny';
        }
    };

    onMount(() => {
        tryReconnect();
    });

    const _connect = () => {
        if (name.length === 0) {
            const jwtPresent = localStorage.getItem('auth-token') !== null;
            if (jwtPresent) {
                tryReconnect();
            }
            else {
                error = 'Please provide a device name';
            }
        }
        else {
            connect({ device_name: name }, onInit, onReject, onJwt, onAccept, onClose);
        }
    };
</script>

{#if connected || reconnect === 'pending'}
<Controller />
{#if reconnect === 'pending'}
<div id="reconnect" />
{/if}
{:else}
<div id="root">
    <div id="title">
        Macropinna Remote
    </div>
    {#if !code}
    <div id="connect">
        <div id="name-input">
            <TextInput
                bind:value={name}
                onSubmit={_connect}
                placeholder="Give your device a name"
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

    #reconnect {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.2);
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