<script lang="ts">
	import { onMount } from "svelte";
	import { Direction, joystick, type Component } from "$lib/joystick";
	import NavLabel from "./NavLabel.svelte";

    export let id: string;
    export let component: Component = {};
    export let value: string;
    export let placeholder: string = '';
    export let validate: (value: string) => boolean = () => true;

    const focusId = joystick.focusId(id);

    onMount(() => {
        joystick.register(id, {
            ...component,
            enter: joystick.focusEnter(focusId)
        });

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

<div id="keyboard-input">
    <NavLabel id={id}>
        <input
            id={focusId}
            type='text'
            bind:value
            placeholder={placeholder}
            on:input={_onChange}
            on:blur={joystick.goFromCb(focusId, Direction.Exit)}
        />
    </NavLabel>
</div>

<style>
    #keyboard-input {
        display: inline-block;
    }

    input {
        background-color: var(--bg2);
        outline: none;
        border: none;
        padding: var(--sm);

        font-size: inherit;
        font-family: inherit;
        font-weight: inherit;
        text-align: inherit;
        color: inherit;
        letter-spacing: inherit;
    }
</style>