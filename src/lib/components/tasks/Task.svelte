<script lang="ts">
  import { TaskModel } from "../../../logic/task";
  import type { TaskRes } from "../../types/tasks";

  export let task: TaskModel;
  export let top: number;

  const clickEvent = () => {
    alert("Clicked");
  };
</script>

<!-- The -20 is to accomidate the padding top -->
<div class="task-container" style="margin-top: {task.top - top}px;">
  <div
    on:click={clickEvent}
    style="background-color: {task.color}; height: {task.height - 20}px;"
    class="task-details"
  >
    {task.body}
  </div>
</div>

<style>
  .task-container {
    display: flex;
  }
  .task-details {
    padding-top: 20px;
    padding-left: 30px;
    font-weight: bold;
    border-radius: 6px;
    text-align: start;
    width: 100%;
  }
</style>
