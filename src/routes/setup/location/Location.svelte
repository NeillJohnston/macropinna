<script lang="ts">
	import { onMount } from "svelte";
	import type { ScreenProps } from "../+page.svelte";
	import { joystick } from "$lib/joystick";
	import Icon from "@iconify/svelte";
	import MenuSection from "../../ui/MenuSection.svelte";
	import KeyboardInput from "../../ui/KeyboardInput.svelte";
	import ContinueButton from "../ContinueButton.svelte";
	import { getRemoteServerIp } from "$lib/api";
	import Button from "../../ui/Button.svelte";
	import RevertButton from "../RevertButton.svelte";

    export let props: ScreenProps;

    const fetchLocButtonId = 'location';
    const latInputId = 'location/lat';
    const longInputId = 'location/long';
    const continueButtonId = 'location/continue';
    const revertButtonId = 'location/revert';

    let latValue = '';
    let longValue = '';
    onMount(() => {
        reset();
    });

    const reset = () => {
        latValue = props.config.weather?.lat.toString() ?? '';
        longValue = props.config.weather?.long.toString() ?? '';
    };

    let fetchLocState: 'fetching' | 'ok' | 'err' = 'ok';

    const fetchLoc = () => {
        fetchLocState = 'fetching';

        _fetchLoc()
            .then(res => {
                if (!res) {
                    fetchLocState = 'err';
                    return;
                }
                
                const [lat, long] = res;
                latValue = lat.toString();
                longValue = long.toString();

                fetchLocState = 'ok';
            })
            .catch(err => {
                console.error(err);
                fetchLocState = 'err';
            });
    }

    const _fetchLoc = async () => {
        const res = await fetch(`https://ipapi.co/json/`);
        const { latitude, longitude } = await res.json();
        return [latitude, longitude] as [number, number];
    };

    const validate = (value: string) => (
        Number.isFinite(parseFloat(value))
    );

    $: invalid = !validate(latValue) || !validate(longValue);

    // TODO figure out something better than hardcoding pixel values
    let mapHeightPx = 6000;
    $: mapWidthPx = 1.33 * mapHeightPx;
    let mapYPx = 0;
    let mapXPx = 0;

    $: {
        if (!invalid) {
            const lat = parseFloat(latValue);
            const long = parseFloat(longValue);

            mapXPx = ((long + 180)/360 + -0.0270) * mapWidthPx;
            // Ref: https://en.wikipedia.org/wiki/Miller_cylindrical_projection
            const y = 5/4 * Math.asinh(Math.tan((-lat * Math.PI / 180) * 4/5));
            mapYPx = (y * 0.23 + 0.4933) * mapHeightPx;
        }
    }

    const saveAndContinue = () => {
        // TODO lat/long will eventually be moved
        if (props.config.weather) {
            props.config.weather.lat = parseFloat(latValue);
            props.config.weather.long = parseFloat(longValue);
        }
        props.next();
    };
</script>

<div id="location">
    <div id="map-height-container">
        <div id="map-container">
            <!-- Three copies of the map - one for the display, and one for each of the left/right edges -->
            <img
                src="/map.svg"
                alt="World map"
                style:height={`${mapHeightPx}px`}
                style:width={`${mapWidthPx}px`}
                style:top={`calc(50% - ${mapYPx}px)`}
                style:left={`calc(50% - ${mapXPx}px)`}
            />
            <img
                src="/map.svg"
                alt="World map (copy)"
                style:height={`${mapHeightPx}px`}
                style:width={`${mapWidthPx}px`}
                style:top={`calc(50% - ${mapYPx}px)`}
                style:left={`calc(50% - ${mapXPx - mapWidthPx}px)`}
            />
            <img
                src="/map.svg"
                alt="World map (copy) (copy)"
                style:height={`${mapHeightPx}px`}
                style:width={`${mapWidthPx}px`}
                style:top={`calc(50% - ${mapYPx}px)`}
                style:left={`calc(50% - ${mapXPx + mapWidthPx}px)`}
            />
            <div id="pin">
                <Icon icon='map:map-pin' />
            </div>
        </div>
        <div id="revert-spacer" />
    </div>
    <div id="menu-container">
        <MenuSection label='Location'>
            <p class="info"><em>
                Automatic geolocation works by IP - if you're using a VPN you can either disable it temporarily or input your location manually.
            </em></p>
            <div class="space" />
            <Button
                id={fetchLocButtonId}
                component={{
                    up: { action: props.prev },
                    down: { id: latInputId }
                }}
                onPress={fetchLoc}
            >
                <!-- TODO abstract this part for future use -->
                <div class="wait-icon" class:on={fetchLocState === 'fetching'}>
                    <Icon icon='carbon:hourglass' inline />
                </div>Get location by IP
            </Button>
            <div class="space" />
            {#if fetchLocState === 'err'}
            <p class="info">Error: could not get location by IP.</p>
            {/if}
        </MenuSection>
        <MenuSection label='Latitude/longitude'>
            <KeyboardInput
                id={latInputId}
                component={{
                    up: { id: fetchLocButtonId },
                    down: { id: longInputId }
                }}
                bind:value={latValue}
                placeholder={'Latitude'}
            />
            <div class="space" />
            <KeyboardInput
                id={longInputId}
                component={{
                    up: { id: latInputId },
                    down: { id: continueButtonId }
                }}
                bind:value={longValue}
                placeholder={'Longitude'}
            />
        </MenuSection>
    </div>
    <RevertButton
        id={revertButtonId}
        component={{
            up: { id: longInputId },
            right: { id: continueButtonId }
        }}
        onPress={reset}
    />
    <ContinueButton
        id={continueButtonId}
        component={{
            up: { id: longInputId },
            left: { id: revertButtonId }
        }}
        onPress={saveAndContinue}
    />
</div>

<style>
    #location {
        width: 100%;
        height: 100%;
        display: flex;
    }

    #map-height-container {
        width: 50%;
        height: 100%;
        display: flex;
        flex-direction: column;
    }

    #revert-spacer {
        height: 2em;
    }

    #map-container {
        width: 100%;
        flex: 1;
        border: 1px solid var(--fg);
        overflow: hidden;
        position: relative;
    }

    #menu-container {
        width: 50%;
        margin-left: var(--md);
    }

    #pin {
        position: absolute;
        width: 100%;
        bottom: 50%;
        text-align: center;
        line-height: 0;
        font-size: var(--f1);
    }

    img {
        position: absolute;
        transition:
            top cubic-bezier(0.2, 1, 0.4, 1) 0.8s,
            left cubic-bezier(0.2, 1, 0.4, 1) 0.8s;
    }

    .space {
        height: var(--sm);
    }

    .info {
        font-size: var(--f-1);
    }

    .wait-icon {
        line-height: 0;
        vertical-align: -0.125em;
        display: inline-block;
        overflow: hidden;

        margin-right: 0;
        width: 0;
        transition:
            width ease 0.4s,
            margin-right ease 0.4s;
    }

    .wait-icon.on {
        width: 2ex;
        margin-right: var(--sm);
    }

    p {
        margin: 0;
    }
</style>