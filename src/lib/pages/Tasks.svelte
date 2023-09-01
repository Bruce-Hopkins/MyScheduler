<script lang="ts">
  import Button from "../components/common/Button.svelte";
  import Modal from "../components/common/Modal.svelte";
  import Layout from "../components/layout/Layout.svelte";
  import AvailableTasks from "../components/tasks/AvailableTasks.svelte";
  import TaskCreateModal from "../components/tasks/TaskCreateModal.svelte";

  let modalIsOpen = false;
  const openCreateModal = () => {
    modalIsOpen = true;
  };
  const dismissModal = () => {
    modalIsOpen = false;
  };
</script>

<Layout>
  <Modal isOpen={modalIsOpen} onDismiss={dismissModal}>
    <TaskCreateModal />
  </Modal>
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
