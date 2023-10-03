<script lang="ts">
    import Icon from '@iconify/svelte';
    import Button from './ui/Button.svelte';
	import { connection } from '$lib/api';

    let value = '';
    let placeholder = '';
    let mode: 'buffered' | 'immediate' = 'buffered';

    const sendImmediate = () => {
        $connection?.send({ Text: value });

        const len = value.length;
        placeholder += value;
        value = '';

        setTimeout(() => {
            placeholder = placeholder.slice(len);
        }, 400);
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

    const onKey = (event: any) => {
        if (mode === 'immediate') {
            const key = event.key;
            if (key === 'Backspace') {
                $connection?.send({ Keyboard: 'Backspace' });
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
</script>

<div id="keyboard">
    <!-- Not using the UI TextInput because this might require special styling/
    features that don't need to be supported there -->
    <div id="text-input">
        <form on:submit={onSubmit}>
            <input
                type="text"
                bind:value
                on:input={onInput}
                on:keydown={onKey}
                placeholder={placeholder}
            />
        </form>
    </div>
    <div id="space" />
    <Button onClick={switchMode} >
        <div id="mode-width">
            <Icon icon={{
                buffered: 'carbon:return',
                immediate: 'carbon:arrow-right'
            }[mode]} inline />
        </div>
    </Button>
</div>

<style>
    #keyboard {
        display: flex;
        padding: 8px;
    }

    #text-input {
        padding: 4px;
        background-color: var(--bg2);
        flex: 1;
    }

    #space {
        width: 8px;
    }

    #mode-width {
        width: calc(1em + 4px);
        text-align: center;
    }

    input {
        background-color: rgba(0, 0, 0, 0);
        border: none;
        outline: none;

        color: inherit;
        font: inherit;
    }
</style>