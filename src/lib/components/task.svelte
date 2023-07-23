<script lang="ts">
  import { Task } from "../../logic/task";
  import type { TaskRes } from "../types/tasks";

  export let task: TaskRes;
  const taskModel = new Task(task);
</script>

<div
  style="background-color: {taskModel.color}; height: {taskModel.height}px;"
  class="task"
>
  {taskModel.body}
</div>

<style>
  .task {
    font-weight: bold;
    border-radius: 6px;
  }
</style>
