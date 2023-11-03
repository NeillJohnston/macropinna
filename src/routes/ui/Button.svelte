<script lang="ts">
	import { joystick, type Component } from "$lib/joystick";
	import { onMount } from "svelte";
	import NavBox from "./NavBox.svelte";

    // This UI element's nav id.
    export let id: string;
    // Partial navigation for the UI element. Note that enter will be overridden.
    export let component: Component = {};
    // Callback for when the button is pressed.
    export let onPress: () => void;

    let pressed = false;
    onMount(() => {
        joystick.register(id, {
            ...component,
            enter: {
                keep: true,
                action: () => {
                    onPress();

                    pressed = true;
                    setTimeout(() => {
                        pressed = false;
                    }, 50);
                }
            }
        });
    });
</script>

<div id="button" class:pressed={pressed}>
    <NavBox id={id}>
        <div id="inner">
            <slot />
        </div>
    </NavBox>
</div>

<style>
    #button {
        display: inline-block;
        text-align: center;
        background-color: var(--bg2);
        transition: background-color ease 0.2s;
    }

    #button.pressed {
        background-color: var(--fg2);
        transition: background-color ease 0.0s;
    }

    #inner {
        padding: var(--xs);
    }
</style>