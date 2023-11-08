<script lang="ts">
	import type { Component } from "$lib/joystick";
	import Icon from "@iconify/svelte";
	import Button from "../../ui/Button.svelte";

    export let id: string;
    export let component: Component = {};
    export let onPress: () => void;
</script>

<div id="edit-button">
    <Button
        id={id}
        component={component}
        onPress={onPress}
    >
        <Icon icon='carbon:edit' inline />
    </Button>
</div>

<style>
    #edit-button {
        position: absolute;
        top: var(--sm);
        right: var(--sm);
        font-size: var(--f-1);
    }
</style>