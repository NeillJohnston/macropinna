<script lang="ts">
	import type { XAlign } from "$lib/layout";
	import Icon from "@iconify/svelte";

    export let props: {
        xAlign: XAlign;
    };

    const list = [
        {
            task: 'Argue with strangers on the internet',
            done: true
        },
        {
            task: 'Make this component',
            done: false
        },
    ];
</script>

<div id="todo">
    <p id="title"><strong>Today's tasks:</strong></p>
    <ul>
        {#each list as item}
        <p>
            <Icon icon={item.done ? 'carbon:checkbox-checked-filled' : 'carbon:checkbox'} inline />
            <span class:crossed={item.done}>{item.task}</span>
        </p>
        {/each}
    </ul>
</div>

<style>
    #title {
        font-size: 1.00rem;
    }

    p {
        margin: 0;
    }

    ul {
        font-size: 0.71rem;
    }

    ul p {
        margin-bottom: 0.5em;
    }

    .crossed {
        text-decoration: line-through;
    }
</style>