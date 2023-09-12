<script lang="ts">
	import Screen from './Screen.svelte';
    import Home from './home/Home.svelte';

    let screenIndex = 1;

    const shift = (delta: number) => {
        screenIndex += delta;
    };
</script>

<div id="page" style={`top: ${-screenIndex * 100}vh;`}>
    <Screen
        onUp={() => shift(-1)}
        onDown={() => shift(1)}
        top
    >
        <div class="under-construction">
            <div style="height: 50%;" />
            <p>(Under construction)</p>
        </div>
    </Screen>
    <Screen
        onUp={() => shift(-1)}
        onDown={() => shift(1)}
    >
        <Home />
    </Screen>
    <Screen
        onUp={() => shift(-1)}
        onDown={() => shift(1)}
        bottom
    >
        <div class="under-construction">
            <div style="height: 50%;" />
            <p>(Under construction)</p>
        </div>
    </Screen>
</div>

<style>
    #page {
        position: relative;
        transition: top cubic-bezier(0.2, 1, 0.4, 1) 0.4s;
    }

    .under-construction {
        width: 100%;
        height: 100%;
        text-align: center;
        display: flex;
        flex-direction: column-reverse;
    }

    .under-construction p {
        font-size: 0.71rem;
    }
</style>