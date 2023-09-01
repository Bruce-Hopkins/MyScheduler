<script lang="ts">
  import {
    get_all_tasks,
    create_task,
    get_tasks_by_day,
  } from "../api/tasks-api";
  import { addDaysToDate } from "../common/calculations";
  import Button from "../components/common/Button.svelte";
  import Modal from "../components/common/Modal.svelte";
  import Taskgroup from "../components/common/Taskgroup.svelte";
  import Layout from "../components/layout/Layout.svelte";
  import Schedule from "../components/tasks/Schedule.svelte";
  import CurrentDateTime from "../components/time/CurrentDateTime.svelte";
  import type { TaskRes, TaskCreate } from "../types/tasks";

  let tasks: TaskRes[][] = [];
  let today = new Date();
  async function get_tasks() {
    console.log("Tasks are", tasks);
    tasks = await get_tasks_by_day(today);
  }
  get_tasks();
  let modalIsOpen = false;
  const openModal = () => {
    modalIsOpen = true;
  };
  const dismissModal = () => {
    modalIsOpen = false;
  };

  const setToPreviousDay = () => {
    today = addDaysToDate(today, -1);
  };

  const setNextDay = () => {
    today = addDaysToDate(today, 1);
  };

  // const start_at = new Date();
  // start_at.setMinutes(start_at.getMinutes() + 1);
  // const task: TaskCreate = {
  //   body: "Test body 2",
  //   start_at: new Date().toISOString(),
  //   end_at: start_at.toISOString(),
  //   colors: "#569BCC",
  // };
  // create_task(task);
</script>

<Layout>
  <main>
    <div class="button-group">
      <Button width={"180px"} onClick={setToPreviousDay}>Previous Day</Button>
      <CurrentDateTime date={today} />
      <Button width={"150px"} onClick={setNextDay}>Next Day</Button>
    </div>
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
    position: absolute;
    width: 100%;
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
