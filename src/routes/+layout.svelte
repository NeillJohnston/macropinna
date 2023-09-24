<script lang="ts">
    import { Direction, joystick } from '$lib/joystick';
	import { onMount } from 'svelte';

    const handleRouting = (event: KeyboardEvent) => {
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

        return () => {
            document.removeEventListener('keydown', handleRouting);
        }
    })
</script>

<div id='app-root'>
    <!-- <div class='dim-edge' /> -->
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

    .mono {
        font-family: 'IBM Plex Mono';
    }

    :root {
        color: #fff;
        font-family: 'IBM Plex Sans';
        /*
        Font sizing is mostly based on rem units, which are based on this number.
        For consistency, I'm using powers of sqrt(2) as the rem multiples, so:
          - 0.50rem
          - 0.71rem
          - 1.00rem
          - 1.41rem
          - 2.00rem
          - 2.83rem
          - 4.00rem
        Can't imagine needing anything smaller/larger than these.
         */
        font-size: 32px;
    }

    #app-root {
        position: fixed;
        width: 100%;
        height: 100%;
        background-color: #000;
        outline: none;
    }
</style>