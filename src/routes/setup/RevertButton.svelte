<script lang="ts">
	import Icon from "@iconify/svelte";
    import Button from "../ui/Button.svelte";
	import type { Component } from "$lib/joystick";

    export let id: string;
    export let component: Component;
    export let onPress: () => void;
</script>

<div id="continue-button">
    <Button
        id={id}
        component={component}
        onPress={onPress}
    >
        <Icon icon={'carbon:undo'} inline /> Revert changes
    </Button>
</div>

<style>
    #continue-button {
        position: absolute;
        bottom: var(--md);
        left: var(--md);
    }
</style>