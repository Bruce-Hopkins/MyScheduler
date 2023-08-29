<script lang="ts">
  import { get_available_tasks } from "../../api/tasks-api";
  import type { TaskRes } from "../../types/tasks";
  import TaskView from "./TaskView.svelte";

  let tasks: TaskRes[] = [];
  get_available_tasks().then((t) => {
    tasks = t;
  });
</script>

<div>
  {#each tasks as task}
    <TaskView {task} />
  {/each}
</div>
