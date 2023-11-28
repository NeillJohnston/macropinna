<script lang="ts">
	import Icon from "@iconify/svelte";
    import Button from "../ui/Button.svelte";
    import type { ScreenProps } from "./+page.svelte";
	import { config, setConfig } from "$lib/api";

    export let props: ScreenProps;

    const saveButtonId = 'finish';
    const cancelButtonId = 'finish/cancel';

    const saveAndExit = () => {
        props.config.needs_setup = undefined;
        setConfig(props.config);
        window.location.href = '/';
    };

    const cancelAndExit = () => {
        $config.needs_setup = undefined;
        setConfig($config);
        window.location.href = '/';
    }
</script>

<div id="finish">
    <Button
        id={saveButtonId}
        component={{
            up: { action: props.prev },
            down: { id: cancelButtonId }
        }}
        onPress={saveAndExit}
    >
        <Icon icon='carbon:save' inline /> Save changes and exit
    </Button>
    <div class="space" />
    <Button
        id={cancelButtonId}
        component={{
            up: { id: saveButtonId }
        }}
        onPress={cancelAndExit}
    >
        <Icon icon='carbon:close' inline /> Discard changes and exit
    </Button>
</div>

<style>
    #finish {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }

    .space {
        height: var(--lg);
    }
</style>