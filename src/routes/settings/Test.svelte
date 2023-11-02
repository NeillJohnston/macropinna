<script lang="ts">
	import { Direction, joystick } from "$lib/joystick";
	import { onMount } from "svelte";
	import CarouselSelector from "../ui/CarouselSelector.svelte";
	import Icon from "@iconify/svelte";
	import Button from "../ui/Button.svelte";
	import Checkbox from "../ui/Checkbox.svelte";
	import MenuSection from "../ui/MenuSection.svelte";

    // TODO remove this. It's just a testing ground for UI.

    onMount(() => {
    });

    let carouselIndex = 0;
    let carousel2Index = 0;
    const carouselValues = [
        'Foo',
        'Bar but with a long name',
        'Baz'
    ];

    let checkboxOn = false;
    let checkbox2On = false;
</script>

<div id="test">
    <MenuSection label='Carousels'>
        <CarouselSelector
            id='test/carousel'
            component={{
                down: { id: 'test/carousel2' },
                exit: {}
            }}
            bind:index={carouselIndex}
            values={carouselValues}
        />
        <div class="space" />
        <CarouselSelector
            id='test/carousel2'
            component={{
                up: { id: 'test/carousel' },
                down: { id: 'test/button' },
                exit: {}
            }}
            bind:index={carousel2Index}
            values={carouselValues}
        />
    </MenuSection>
    <div class="space" />
    <MenuSection label='Buttons'>
        <Button
            id='test/button'
            component={{
                up: { id: 'test/carousel2' },
                down: { id: 'test/button2' },
                exit: {}
            }}
            onPress={() => {}}
        >
            Button
        </Button>
        <div class="space" />
        <Button
            id='test/button2'
            component={{
                up: { id: 'test/button' },
                down: { id: 'test/checkbox' },
                exit: {}
            }}
            onPress={() => {}}
        >
            Another Button
        </Button>
    </MenuSection>
    <div class="space" />
    <MenuSection label='Checkboxes'>
        <div style:display="flex">
            <Checkbox
                id='test/checkbox'
                bind:on={checkboxOn}
                component={{
                    up: { id: 'test/button2' },
                    right: { id: 'test/checkbox2' },
                    exit: {}
                }}
            >
                Checkbox
            </Checkbox>
            <div class="hspace" />
            <Checkbox
                id='test/checkbox2'
                bind:on={checkbox2On}
                component={{
                    up: { id: 'test/button2' },
                    left: { id: 'test/checkbox' },
                    exit: {}
                }}
            >
                Another Checkbox
            </Checkbox>
        </div>
    </MenuSection>
</div>

<style>
    #test {
        width: 100%;
        height: 100%;
    }

    .space {
        height: var(--lg);
    }

    .hspace {
        width: var(--lg);
    }
</style>