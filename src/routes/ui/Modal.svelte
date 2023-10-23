<div id="modal">
    <slot />
</div>

<style>
    #modal {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        z-index: 1;
        background-color: rgba(0, 0, 0, 0.5);
    }
</style>