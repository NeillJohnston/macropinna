<script lang="ts">
	import { joystick, type Component } from "$lib/joystick";
	import { onMount } from "svelte";
	import NavBox from "./NavBox.svelte";
	import Icon from "@iconify/svelte";

    // This UI element's nav id.
    export let id: string;
    // Partial navigation for the UI element. Note that enter will be overridden.
    export let component: Component = {};
    // Whether this checkbox is on or not.
    export let on: boolean;
    // Optional callback for when the checkbox state changes.
    export let onChange: (on: boolean) => void = () => {};

    $: icon = on ? 'carbon:checkbox-checked-filled' : 'carbon:checkbox';

    onMount(() => {
        joystick.register(id, {
            ...component,
            enter: {
                keep: true,
                action: () => {
                    on = !on;
                    onChange(on);
                }
            },
            scrollTo: '#' + id
        });
    });
</script>

<div id="checkbox">
    <div id={id} />
    <NavBox id={id}>
        <div id="inner">
            <Icon icon={icon} inline />
            <slot />
        </div>
    </NavBox>
</div>

<style>
    #checkbox {
        display: inline-block;
        text-align: center;
    }

    #inner {
        padding: var(--xs);
    }
</style>