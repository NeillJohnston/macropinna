<script lang="ts">
    export let onClick: () => void;
</script>

<div
    id="button"
    on:click={onClick}
    on:keypress={onClick}
    role="button" tabindex="0"
>
    <slot />
</div>

<style>
    #button {
        display: inline-block;
        padding: 4px;
        border: 1px solid var(--fg);
        transition: background-color ease-out 0.2s;
    }

    #button:active {
        background-color: var(--bg2);
        transition: background-color ease 0.0s;
    }
</style>