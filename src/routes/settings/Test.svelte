<script lang="ts">
	import { Direction, joystick } from "$lib/joystick";
	import { onMount } from "svelte";
	import CarouselSelector from "../ui/CarouselSelector.svelte";
	import Icon from "@iconify/svelte";
	import Button from "../ui/Button.svelte";

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
    <CarouselSelector
        id='test/carousel'
        component={{
            down: { id: 'test/button' },
            exit: {}
        }}
        bind:index={carouselIndex}
        values={carouselValues}
    />
    <Button
        id='test/button'
        component={{
            up: { id: 'test/carousel' },
            exit: {}
        }}
        onPress={() => {}}
    >
        Button
    </Button>
</div>

<style>
    #test {
        width: 100%;
        height: 100%;
    }
</style>