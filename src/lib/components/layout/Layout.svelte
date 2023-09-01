<script>
  import Sidebar from "./Sidebar.svelte";
  import Topnav from "./Topnav.svelte";
</script>

<div class="layout">
  <Topnav />
  <div class="content">
    <Sidebar />
    <div class="main-container">
      <slot />
    </div>
  </div>
</div>

<style>
  .layout {
    display: flex;
    flex-direction: column;
    width: 100vw;
    height: 100vh;
  }
  .content {
    display: flex;
    flex-direction: row;
    justify-content: start;
    height: 100%;
  }
  .main-container {
    width: 100%;
    position: relative;
  }
</style>
