<script lang="ts">
	import { onMount } from "svelte";
    import type { ScreenProps } from "../+page.svelte";
	import CarouselSelector from "../../ui/CarouselSelector.svelte";
	import { colorThemes, setColorTheme, setStyleTheme, styleThemes } from "$lib/themes";
	import ContinueButton from "../ContinueButton.svelte";
	import RevertButton from "../RevertButton.svelte";

    export let props: ScreenProps;

    const colorSelectorId = 'theme';
    const styleSelectorId = 'theme/style';
    const continueButtonId = 'theme/continue';
    const revertButtonId = 'location/revert';

    let colorSelectorIndex = 0;
    let styleSelectorIndex = 0;
    onMount(() => {
        reset();
    });

    const colorSelectorValues = colorThemes.map(theme => theme.name);
    const styleSelectorValues = styleThemes.map(theme => theme.name);

    const reset = () => {
        colorSelectorIndex = colorThemes.findIndex(theme => (
            theme.name === props.config.theme.color
        ));
        styleSelectorIndex = styleThemes.findIndex(theme => (
            theme.name === props.config.theme.style
        ));

        if (colorSelectorIndex === -1) {
            colorSelectorIndex = 0;
        }
        if (styleSelectorIndex === -1) {
            styleSelectorIndex = 0;
        }

        changeColorTheme(colorSelectorIndex);
        changeStyleTheme(styleSelectorIndex);
    };

    const saveAndContinue = () => {
        props.config.theme.color = colorThemes[colorSelectorIndex].name;
        props.config.theme.style = styleThemes[styleSelectorIndex].name;

        props.next();
    };

    const changeColorTheme = (index: number) => {
        setColorTheme(colorThemes[index]);
    };

    const changeStyleTheme = (index: number) => {
        setStyleTheme(styleThemes[index]);
    }
</script>

<div id="theme">
    <CarouselSelector
        id={colorSelectorId}
        component={{
            up: { action: props.prev },
            down: { id: styleSelectorId }
        }}
        bind:index={colorSelectorIndex}
        values={colorSelectorValues}
        onChange={changeColorTheme}
    />
    <div class="space" />
    <CarouselSelector
        id={styleSelectorId}
        component={{
            up: { id: colorSelectorId },
            down: { id: continueButtonId }
        }}
        bind:index={styleSelectorIndex}
        values={styleSelectorValues}
        onChange={changeStyleTheme}
    />
</div>
<RevertButton
    id={revertButtonId}
    component={{
        up: { id: styleSelectorId },
        right: { id: continueButtonId }
    }}
    onPress={reset}
/>
<ContinueButton
    id={continueButtonId}
    component={{
        up: { id: styleSelectorId },
        left: { id: revertButtonId }
    }}
    onPress={saveAndContinue}
/>

<style>
    #theme {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        font-size: var(--f0);
    }

    .space {
        height: var(--md);
    }
</style>