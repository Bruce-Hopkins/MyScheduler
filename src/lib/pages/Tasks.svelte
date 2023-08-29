<script lang="ts">
  import Button from "../components/common/Button.svelte";
  import Layout from "../components/layout/Layout.svelte";
  import AvailableTasks from "../components/tasks/AvailableTasks.svelte";
  import DaySelectors from "../components/tasks/DaySelectors.svelte";

  const openCreateModal = () => {};
</script>

<Layout>
  <div class="button-container">
    <div class="button-group">
      <Button onClick={openCreateModal}>New</Button>
    </div>
  </div>
  <div class="tasks-container">
    <AvailableTasks />
  </div>
</Layout>

<style>
  .button-container {
    display: flex;
    justify-content: end;
    width: 90%;
    margin: 50px auto;
  }
  .tasks-container {
    width: 750px;
    margin: 0 auto;
  }
</style>
