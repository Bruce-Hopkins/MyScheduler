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
<div
  on:click={clickEvent}
  style="background-color: {taskModel.color}; height: {taskModel.height -
    20}px;"
  class="task"
>
  {taskModel.body}
</div>

<style>
  .task {
    padding-top: 20px;
    padding-left: 30px;
    font-weight: bold;
    border-radius: 6px;
    text-align: start;
  }
</style>
