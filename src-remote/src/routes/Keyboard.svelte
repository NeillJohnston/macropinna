<script lang="ts">
    import Icon from '@iconify/svelte';
    import Button from './ui/Button.svelte';
	import { connection } from '$lib/api';

    let value = '';
    let placeholder = '';
    let mode: 'buffered' | 'immediate' = 'buffered';

    const pushPlaceholder = (pushed: string) => {
        placeholder += pushed;
        setTimeout(() => {
            placeholder = placeholder.slice(pushed.length);
        }, 800);
    }

    let p = '';

    const sendImmediate = () => {
        $connection?.send({ Text: value });

        pushPlaceholder(value);
        value = '';
    };

    const switchMode = () => {
        // @ts-ignore
        mode = {
            buffered: 'immediate',
            immediate: 'buffered'
        }[mode];

        if (mode === 'immediate') {
            sendImmediate();
        }
    };

    const onKey = (event: KeyboardEvent) => {
        // TODO this doesn't work for me because of my keyboard, and I suspect
        // that other keyboards will be similar. Leaving it here because it
        // won't actually harm anything
        if (mode === 'immediate') {
            const key = event.key;
            if (key === 'Backspace') {
                sendBackspace();
            }
        }
    }

    const onInput = () => {
        if (mode === 'immediate') {
            sendImmediate();
        }
    };

    const onSubmit = () => {
        $connection?.send({ Text: value + '\n' });
        value = '';
    };

    const sendBackspace = () => {
        $connection?.send({ Keyboard: 'Backspace' });

        pushPlaceholder('\u232B');
    }
</script>

<div id="keyboard">
    <!-- Not using the UI TextInput because this might require special styling/
    features that don't need to be supported there -->
    <Button onClick={switchMode} >
        <div id="mode-width">
            <Icon icon={{
                buffered: 'carbon:return',
                immediate: 'carbon:arrow-right'
            }[mode]} inline />
        </div>
    </Button>
    <div id="text-input">
        {#if mode === 'buffered'}
        <form on:submit={onSubmit}>
            <input
                type="text"
                bind:value
                on:input={onInput}
                on:keydown={onKey}
                placeholder={placeholder}
            />
        </form>
        {:else if mode === 'immediate'}
        <input
            type="password"
            bind:value
            on:input={onInput}
            on:keydown={onKey}
            placeholder={placeholder}
        />
        {/if}
    </div>
    {#if mode === 'immediate'}
    <!-- Prevents this button from blurring the input -->
    <div
        on:mousedown={(event) => { event.preventDefault(); }}
        role="button"
        tabindex="0"
    >
        <Button onClick={sendBackspace} >
            <div id="mode-width">
                <Icon icon='carbon:delete' inline />
            </div>
        </Button>
    </div>
    {/if}
</div>

<style>
    #keyboard {
        position: fixed;
        bottom: 0;
        width: 100%;
        box-sizing: border-box;
        display: flex;
        padding: 8px;
        background-color: var(--bg);
        z-index: 1;
    }

    #text-input {
        padding: 4px;
        background-color: var(--bg2);
        flex: 1;
    }

    #mode-width {
        width: calc(1em + 4px);
        text-align: center;
    }

    input {
        background-color: rgba(0, 0, 0, 0);
        border: none;
        outline: none;
        size: 1;

        color: inherit;
        font: inherit;
    }
</style>