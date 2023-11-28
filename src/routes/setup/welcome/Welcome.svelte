<script lang="ts">
	import { onMount } from "svelte";
	import type { ScreenProps } from "../+page.svelte";
    import KeyboardInput from "../../ui/KeyboardInput.svelte";
	import ContinueButton from "../ContinueButton.svelte";

    export let props: ScreenProps;

    const nameInputId = 'welcome';
    const continueButtonId = 'welcome/continue';

    let nameValue = '';
    onMount(() => {
        nameValue = props.config.name;
    });

    const saveAndContinue = () => {
        props.next();
    }
</script>

<script context="module">
    export const ID = 'welcome';
</script>

<div id="welcome">
    <p><strong>Welcome to Macropinna</strong></p>
    <KeyboardInput
        id={nameInputId}
        component={{
            down: { id: continueButtonId }
        }}
        bind:value={nameValue}
        placeholder={'What should I call you?'}
    />
</div>
<ContinueButton
    id={continueButtonId}
    component={{
        up: { id: nameInputId }
    }}
    onPress={saveAndContinue}
/>

<style>
    #welcome {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
    }
</style>