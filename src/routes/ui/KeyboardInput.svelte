<script lang="ts">
	import { onMount } from "svelte";
	import { Direction, joystick, type Component } from "$lib/joystick";
	import NavLabel from "./NavLabel.svelte";

    // This UI element's nav id.
    export let id: string;
    // Partial navigation for the UI element. Note that enter will be overridden.
    export let component: Component = {};
    // The value that has been typed in the input. Note that this needs to be bound (bind:value={...}).
    export let value: string;
    // Optional placeholder text for when the input is empty.
    export let placeholder: string = '';
    // Set to false to indicate that an error has occurred.
    export let valid = true;
    // A function that filters input so that the value can only ever be in a valid state. Only for very specific use cases, prefer to set the valid flag.
    export let validate: (value: string) => boolean = () => true;

    const focusId = joystick.focusId(id);

    onMount(() => {
        joystick.register(id, {
            ...component,
            enter: joystick.focusEnter(focusId),
            scrollTo: '#' + focusId
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
            class:invalid={!valid}
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

        transition: background-color ease 0.2s;
    }

    .invalid {
        background-color: var(--bg-err);
    }
</style>