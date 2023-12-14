<script lang="ts">
	import { config, listenConfig } from '$lib/api';
    import { Direction, joystick, nav } from '$lib/joystick';
	import { colorThemes, setColorTheme, setStyleTheme, styleThemes } from '$lib/themes';
	import { onMount } from 'svelte';

    const hideCursorAfterMs = 3000;

    let hideCursor = true;
    let hideCursorTimer: number | undefined = undefined;

    const handleRouting = (event: KeyboardEvent) => {
        if (!$nav.endsWith(':focus')) {
            event.preventDefault();
        }

        let dir: Direction;
        switch (event.key) {
            case 'ArrowUp':
                dir = Direction.Up;
                break;
            case 'ArrowDown':
                dir = Direction.Down;
                break;
            case 'ArrowLeft':
                dir = Direction.Left;
                break;
            case 'ArrowRight':
                dir = Direction.Right;
                break;
            case ' ':
                dir = Direction.Enter;
                break;
            case 'Enter':
                dir = Direction.Enter;
                break;
            case 'Escape':
                dir = Direction.Exit;
                break;
            default:
                return;
        }

        joystick.go(dir);
    };

    onMount(() => {
        document.addEventListener('keydown', handleRouting);

        document.addEventListener('mousemove', () => {
            hideCursor = false;

            if (hideCursorTimer !== undefined) {
                clearTimeout(hideCursorTimer);
            }

            hideCursorTimer = setTimeout(() => {
                hideCursor = true;
            }, hideCursorAfterMs);
        });

        listenConfig();

        reloadColorTheme();
        reloadStyleTheme();

        return () => {
            document.removeEventListener('keydown', handleRouting);
        }
    });

    const reloadColorTheme = () => {
        const theme = colorThemes.find(theme => (
            theme.name === $config.theme.color
        ));

        if (theme) {
            setColorTheme(theme);
        }
    }

    const reloadStyleTheme = () => {
        const theme = styleThemes.find(theme => (
            theme.name === $config.theme.style
        ));

        if (theme) {
            setStyleTheme(theme);
        }
    }

    $: {
        $config.theme.color && reloadColorTheme();
    }

    $: {
        $config.theme.style && reloadStyleTheme();
    }
</script>

<div
    id='app-root'
    class:hide-cursor={hideCursor}
>
    <slot />
</div>

<style>
    @font-face {
        font-family: 'IBM Plex Sans';
        font-weight: normal;
        src: url('/fonts/IBMPlexSans-Light.ttf');
    }

    @font-face {
        font-family: 'IBM Plex Sans';
        font-weight: bold;
        src: url('/fonts/IBMPlexSans-Medium.ttf');
    }

    @font-face {
        font-family: 'IBM Plex Mono';
        font-weight: normal;
        src: url('/fonts/IBMPlexMono-Regular.ttf');
    }

    .hide-cursor {
        cursor: none;
    }

    .mono {
        font-family: var(--code);
    }

    :root {
        /* Colors */
        --fg: #ffffff;
        --fg2: #888888;
        --bg: #000000;
        --bg2: #222222;
        --bg-err: #880000;

        /* Margin/padding sizes */
        --xs: 0.13rem;
        --sm: 0.25rem;
        --md: 0.50rem;
        --lg: 1.00rem;
        --xl: 2.00rem;

        /*
        Font sizing - fixed s.t. 1rem ~= 1/24th of the screen. Relative font
        sizes (f1, f2, etc.) are then based on a power-of-2 multiple system.
        */
        font-size: 4.167vh;
        --f-4: 0.25rem;
        --f-3: 0.35rem;
        --f-2: 0.50rem;
        --f-1: 0.71rem;
        --f0:  1.00rem;
        --f1:  1.41rem;
        --f2:  2.00rem;
        --f3:  2.83rem;
        --f4:  4.00rem;

        /* Font families */
        --text: 'IBM Plex Sans';
        --code: 'IBM Plex Mono';

        color: var(--fg);
        font-family: var(--text);
    }

    #app-root {
        position: fixed;
        width: 100%;
        height: 100%;
        background-color: var(--bg);
        outline: none;
        font-size: var(--f0);
    }
</style>