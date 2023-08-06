<script lang="ts">
  import svelteLogo from "./assets/svelte.svg";
  import viteLogo from "/vite.svg";
  import Counter from "./lib/Counter.svelte";
  import type { TaskCreate, TaskRes } from "./lib/types/tasks";
  import { get_all_tasks } from "./lib/api/tasks-api";
  import Task from "./lib/components/tasks/Task.svelte";
  import Schedule from "./lib/components/tasks/Schedule.svelte";
  import Taskgroup from "./lib/components/common/Taskgroup.svelte";
  import Button from "./lib/components/common/Button.svelte";
  import Modal from "./lib/components/common/Modal.svelte";
  // When using the Tauri global script (if not using the npm package)

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
</script>

<main>
  <Button onClick={openModal}>Submit</Button>
  <Modal onDismiss={dismissModal} isOpen={modalIsOpen}>Yoo</Modal>
  <Schedule>
    {#each tasks as task}
      <Taskgroup tasks={task} />
    {/each}
  </Schedule>
</main>

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
