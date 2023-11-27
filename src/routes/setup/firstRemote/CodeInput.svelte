<script lang="ts">
	import KeyboardInput from "../../ui/KeyboardInput.svelte";
	import NavLabel from "../../ui/NavLabel.svelte";
	import type { Component } from "$lib/joystick";

    export let code: string;
    export let component: Component;
    export let onMatch: () => void;

    let value = '';

    const validate = (value: string): boolean => {
        if (!/^\d{0,8}$/.test(value)) return false;

        if (value === code) {
            onMatch();
        }
        return true;
    };
</script>

<div id="code-input" class='mono'>
    <NavLabel id='first-remote/modal:code'>
        <KeyboardInput
            id='first-remote/modal:code'
            component={component}
            bind:value
            validate={validate}
        />
    </NavLabel>
</div>

<style>
    #code-input {
        font-size: 2.00rem;
        font-weight: bold;
        letter-spacing: 0.25em;
        /* text-align: left; */
        margin: 0 1rem;
    }
</style>