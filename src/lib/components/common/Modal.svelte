<script lang="ts">
  export let isOpen: boolean;
  export let onDismiss: () => void;
</script>

{#if isOpen}
  <div class="modal-container">
    <div class="modal-children">
      <slot />
    </div>
    <div class="black-screen" on:click={onDismiss} />
  </div>
{/if}

<style>
  .black-screen {
    width: 100%;
    height: 100%;
    position: absolute;
    background-color: rgba(0, 0, 0, 0.3);
    z-index: 100;
  }
  .modal-children {
    z-index: 101;
    width: 600px;
    margin: 0px auto;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
  .modal-container {
    width: 100vw;
    height: 100vh;
    position: fixed;
    top: 0;
    left: 0;
    z-index: 100;
  }
</style>
