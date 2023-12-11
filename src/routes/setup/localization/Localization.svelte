<script lang="ts">
	import type { ScreenProps } from "../+page.svelte";
	import CarouselSelector from "../../ui/CarouselSelector.svelte";
	import MenuSection from "../../ui/MenuSection.svelte";
	import ContinueButton from "../ContinueButton.svelte";

    export let props: ScreenProps;

    const unitsId = 'localization';
    const continueButtonId = 'localization/continue';

    const units = [
        'Imperial (°F, mi, etc.)',
        'Metric (°C, km, etc.)'
    ];
    let unitsIndex = 0;
</script>

<div id="localization">
    <MenuSection label='Localization'>
        <CarouselSelector
            id={unitsId}
            component={{
                up: { action: props.prev },
                down: { id: continueButtonId }
            }}
            values={units}
            bind:index={unitsIndex}
        />
    </MenuSection>
</div>
<ContinueButton
    id={continueButtonId}
    component={{
        up: { id: unitsId }
    }}
    onPress={props.next}
/>
    
<style>
    #localization {
        width: 100%;
        height: 100%;
    }
</style>