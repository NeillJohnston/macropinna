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
    }

    /* .dim-edge {
        position: absolute;
        width: 100%;
        height: 100%;
        background: linear-gradient(90deg, #000, rgba(0, 0, 0, 0) 10%, rgba(0, 0, 0, 0) 90%, #000);
        z-index: 0;
    }

    .background-content {
        position: relative;
        width: 100%;
        height: 100%;
        z-index: -1;
    }

    .foreground-content {
        position: relative;
        width: 100%;
        height: 100%;
        z-index: -1;
    } */
</style>