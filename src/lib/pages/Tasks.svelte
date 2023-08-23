<script lang="ts">
  import Button from "../components/common/Button.svelte";
  import Layout from "../components/layout/Layout.svelte";
    import DaySelectors from "../components/tasks/DaySelectors.svelte";

  const openCreateModal = () => {

  }
</script>

<Layout>
  <div class="button-container">
    <Button onClick={openCreateModal}>
      New
    </Button>
      <DaySelectors/>
  </div>
</Layout>

<style>
  .button-container{

  }
</style>
