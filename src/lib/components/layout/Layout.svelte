<script>
  import Sidebar from "./Sidebar.svelte";
  import Topnav from "./Topnav.svelte";
</script>

<div class="layout">
  <Topnav />
  <div class="content">
    <Sidebar />
    <slot />
  </div>
</div>

<style>
  .layout {
    width: 100vw;
    height: 100vh;
  }
  .content {
    display: flex;
    justify-content: start;
  }
</style>
