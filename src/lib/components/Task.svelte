<script lang="ts">
  import { Task } from "../../logic/task";
  import type { TaskRes } from "../types/tasks";

  export let task: TaskRes;
  const taskModel = new Task(task);

  const clickEvent = () => {
    alert("Clicked");
  };
</script>

<!-- The -20 is to accomidate the padding top -->
<div class="task-container">
  <div
    on:click={clickEvent}
    style="background-color: white; height: {taskModel.height - 20}px;"
    class="task-details"
  >
    {taskModel.body}
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
