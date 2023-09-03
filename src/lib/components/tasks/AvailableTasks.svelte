<script lang="ts">
  import { get_available_tasks } from "../../api/tasks-api";
  import type { TaskRes } from "../../types/tasks";
  import TaskView from "./TaskView.svelte";

  let tasks: TaskRes[] = [];
  get_available_tasks().then((t) => {
    console.log("Tasks are", t);
    tasks = t;
  });
  let textArray = [];
  for (let i = 0; i < 10000; i++) {
    textArray.push(`Test ${i}`);
  }
</script>

<div class="tasks-group">
  {#each tasks as task}
    <TaskView {task} />
  {/each}
</div>

<style>
  .tasks-group {
    /* overflow-y: auto; */
    position: absolute;
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }
</style>
