<script lang="ts">
	import { listenConfig } from '$lib/api';
    import { Direction, joystick, nav } from '$lib/joystick';
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
            case 'Escape':
                dir = Direction.Exit;
                break;
            default:
                return;
        }

        joystick.go(dir);
    }

    onMount(() => {
        joystick.set(['home']);

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

        return () => {
            document.removeEventListener('keydown', handleRouting);
        }
    })
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
        font-family: 'IBM Plex Mono';
    }

    :root {
        /* Colors */
        --fg: #ffffff;
        --fg2: #888888;
        --bg: #000000;
        --bg2: #222222;

        /* Margin/padding sizes */
        --sm: 0.13rem;
        --md: 0.25rem;
        --lg: 0.50rem;

        color: var(--fg);
        font-family: 'IBM Plex Sans';
        /*
        Font sizing is mostly based on rem units, which are based on this number.
        For consistency, I'm using powers of sqrt(2) as the rem multiples, so:
          - 0.25rem
          - 0.35rem
          - 0.50rem
          - 0.71rem
          - 1.00rem
          - 1.41rem
          - 2.00rem
          - 2.83rem
          - 4.00rem
        Can't imagine needing anything smaller/larger than these.
         */
        font-size: 4.167vh;
    }

    #app-root {
        position: fixed;
        width: 100%;
        height: 100%;
        background-color: var(--bg);
        outline: none;
    }
</style>