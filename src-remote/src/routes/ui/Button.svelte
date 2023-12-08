<script lang="ts">
    export let onClick: () => void;
    export let display = 'inline';
    export let noBlur = false;
</script>

<div
    id="button"
    style:display={display}
    on:click={onClick}
    on:keypress={onClick}
    role="button"
    tabindex="0"
    on:mousedown={noBlur ? ((event) => { event.preventDefault(); }) : undefined }
>
    <slot />
</div>

<style>
    #button {
        padding: 4px;
        border: 1px solid var(--fg);
        transition: background-color ease-out 0.2s;
    }

    #button:active {
        background-color: var(--bg2);
        transition: background-color ease 0.0s;
    }
</style>