<div class="modal">
    <div class="content">
        <slot />
    </div>
</div>

<style>
    .modal {
        position: fixed;
        z-index: 1;
        left: 0;
        top: 0;
        width: 100%;
        height: 100%;
        overflow: auto;
        background-color: rgb(0, 0, 0);
        background-color: rgba(0, 0, 0, 0.4);
    }

    .content {
        background-color: white;
        margin: 15% auto;
        padding: 20px;
        width: 80%;
    }
</style>
