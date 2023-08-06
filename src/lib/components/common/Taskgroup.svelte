<script lang="ts">
  import { TaskModel } from "../../../logic/task";
  import type { TaskRes } from "../../types/tasks";
  import Task from "../tasks/Task.svelte";

  export let tasks: TaskRes[];
  const taskModels = tasks.map((task) => new TaskModel(task));
  const firstTaskModel = taskModels[0];
</script>

<!-- The -20 is to accomidate the padding top -->
<div class="task-group" style="margin-top: {firstTaskModel.top}px">
  {#each taskModels as task}
    <Task top={firstTaskModel.top} {task} />
  {/each}
</div>

<style>
  .task-group {
    display: flex;
  }
</style>
