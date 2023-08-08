<script lang="ts">
  import { get_all_tasks, create_task } from "../api/tasks-api";
  import Button from "../components/common/Button.svelte";
  import Modal from "../components/common/Modal.svelte";
  import Taskgroup from "../components/common/Taskgroup.svelte";
  import Layout from "../components/layout/Layout.svelte";
  import Schedule from "../components/tasks/Schedule.svelte";
  import CurrentDateTime from "../components/time/CurrentDateTime.svelte";
  import type { TaskRes, TaskCreate } from "../types/tasks";

  let tasks: TaskRes[][] = [];
  async function get_tasks() {
    tasks = await get_all_tasks();
  }
  get_tasks();
  let modalIsOpen = false;
  const openModal = () => {
    modalIsOpen = true;
  };
  const dismissModal = () => {
    modalIsOpen = false;
  };

  const start_at = new Date();
  start_at.setMinutes(start_at.getMinutes() + 1);
  const task: TaskCreate = {
    body: "Test body 2",
    start_at: new Date().toISOString(),
    end_at: start_at.toISOString(),
    colors: "#569BCC",
  };
  create_task(task);
</script>

<Layout>
  <main>
    <CurrentDateTime />
    <Button onClick={openModal}>Submit</Button>
    <Modal onDismiss={dismissModal} isOpen={modalIsOpen}>Yoo</Modal>
    <Schedule>
      {#each tasks as task}
        <Taskgroup tasks={task} />
      {/each}
    </Schedule>
  </main>
</Layout>

<style>
  main {
    width: 90vw;
  }
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .read-the-docs {
    color: #888;
  }
</style>
