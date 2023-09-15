<!-- Copy-pasted from the original layout. -->

<div id='app-root'>
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
        font-size: 20px;
    }

    #app-root {
        position: fixed;
        width: 100%;
        height: 100%;
        background-color: #000;
        top: 0;
        left: 0;
    }
</style>