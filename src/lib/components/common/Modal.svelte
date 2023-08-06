<script lang="ts">
  export let isOpen: boolean;
  export let onDismiss: () => void;
</script>

{#if isOpen}
  <div class="modal-container">
    <div class="black-screen" on:click={onDismiss} />
    <div class="modal-children">
      <slot />
    </div>
  </div>
{/if}

<style>
  .black-screen {
    width: 100%;
    height: 100%;
    position: absolute;
    background-color: rgba(0, 0, 0, 0.3);
  }
  .modal-children {
    z-index: 101;
  }
  .modal-container {
    display: flex;
    width: 100vw;
    height: 100vh;
    justify-content: center;
    align-items: center;
    position: fixed;
    top: 0;
    left: 0;
    z-index: 100;
  }
</style>
