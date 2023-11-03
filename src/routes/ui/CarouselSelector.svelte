<script lang="ts">
	import Icon from "@iconify/svelte";
    import NavBox from "./NavBox.svelte";
	import { joystick, type Component } from "$lib/joystick";
	import { onMount } from "svelte";

    // This UI element's nav id.
    export let id: string;
    // Partial navigation for the UI element. Note that left and right will be overridden.
    export let component: Component = {};
    // The currently selected index.
    export let index: number;
    // Optional callback for when the index changes.
    export let onChange: (index: number) => void = () => {};
    // The possible values that can be displayed.
    export let values: string[];
    // Set to true if you want to provide custom rendering for the currently selected item (via this component's slot).
    export let custom = false;

    onMount(() => {
        joystick.register(id, {
            ...component,
            left: {
                keep: true,
                action: () => {
                    index = (index - 1 + values.length) % values.length;
                    onChange(index);
                }
            },
            right: {
                keep: true,
                action: () => {
                    index = (index + 1) % values.length;
                    onChange(index);
                }
            }
        });
    });
</script>

<NavBox id={id}>
    <div id="carousel">
        <!-- Wrapping the icons in a div prevents the text from overflowing onto the icons -->
        <div><Icon icon='carbon:chevron-left' inline /></div>
        <div id='display'>
            {#if !custom}
            {values[index]}
            {:else}
            <slot />
            {/if}
        </div>
        <div><Icon icon='carbon:chevron-right' inline /></div>
    </div>
</NavBox>

<style>
    #carousel {
        display: flex;
        justify-content: space-between;
        align-items: center;
        flex-direction: row;
        padding: var(--xs);
    }

    #display {
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
</style>