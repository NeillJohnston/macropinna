<script lang="ts">
	import { Direction, joystick } from "$lib/joystick";
	import { onMount } from "svelte";
	import CarouselSelector from "../ui/CarouselSelector.svelte";
	import Icon from "@iconify/svelte";

    // TODO remove this. It's just a testing ground for UI.

    onMount(() => {
    });

    let carouselIndex = 0;
    const carouselValues = [
        'Foo',
        'Bar but with a long name',
        'Baz'
    ]
</script>

<div id="test">
    <div style:width='300px'>
        <CarouselSelector
            id='test/carousel'
            component={{
                exit: {}
            }}
            bind:index={carouselIndex}
            values={carouselValues}
            custom
        >
            <div>
                <Icon icon='carbon:accessibility-color' inline />{carouselValues[carouselIndex]}
            </div>
        </CarouselSelector>
    </div>
</div>

<style>
    #test {
        width: 100%;
        height: 100%;
    }
</style>