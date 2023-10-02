<script lang="ts">
	import { onMount } from "svelte";
	import { Direction, joystick } from "$lib/joystick";

    export let parentId: string;
    export let value: string;
    export let placeholder: string = '';
    export let validate: (value: string) => boolean = () => true;

    const focusId = joystick.focusId(parentId);

    onMount(() => {
        joystick.register(focusId, {
            exit: joystick.focusExit(focusId)
        });
    });

    let oldValue = value;
    const _onChange = (event: any) => {
        if (!validate(event.target.value)) {
            value = oldValue;
        }

        oldValue = value;
    }
</script>

<!-- <div id="keyboard-input"> -->
    <input
        id={focusId}
        type='text'
        bind:value
        placeholder={placeholder}
        on:input={_onChange}
        on:blur={joystick.goFromCb(focusId, Direction.Exit)}
    />
<!-- </div> -->

<style>
    /* #keyboard-input {
        display: inline;
    } */

    input {
        background-color: rgba(0, 0, 0, 0);
        outline: none;
        border: none;

        font-size: inherit;
        font-family: inherit;
        font-weight: inherit;
        text-align: inherit;
        color: inherit;
        letter-spacing: inherit;
    }
</style>