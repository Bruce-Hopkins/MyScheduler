<script lang="ts">
  import { TaskModel } from "../../../logic/task";
  import type { TaskRes } from "../../types/tasks";
  import Calendar from "../common/icons/Calendar.svelte";
  import ClockIcon from "../common/icons/ClockIcon.svelte";

  export let task: TaskRes;
  const taskModel = new TaskModel(task);
</script>

<div class="task-container">
  <h2>
    {task.body}
  </h2>
  <div class="inline-block">
    <div class="inline"><ClockIcon /> <span>{taskModel.time}</span></div>
    <div class="inline"><Calendar /> <span>{taskModel.date}</span></div>
  </div>
</div>

<style>
  .task-container {
    background-color: #404040;
    padding: 20px;
  }
  .inline {
    display: inline;
  }
  .inline-block {
    display: inline;
  }
</style>
