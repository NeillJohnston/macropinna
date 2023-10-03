<script lang="ts">
    export let value: string;
    export let onSubmit: () => void = () => {};
    export let placeholder: string | undefined = undefined;
</script>

<div id="text-input">
    <form on:submit={onSubmit}>
        <input
            type="text"
            bind:value={value}
            placeholder={placeholder}
        />
    </form>
</div>

<style>
    #text-input {
        padding: 4px;
        background-color: var(--bg2);
    }

    input {
        background-color: rgba(0, 0, 0, 0);
        border: none;
        outline: none;

        color: inherit;
        font: inherit;
    }
</style>