<script lang="ts">
  import { create_task } from "../../api/tasks-api";
  import { dateToDatetimeISOString } from "../../common/converstions";
  import type { Color } from "../../types/common";
  import type { TaskCreate } from "../../types/tasks";
  import Button from "../common/Button.svelte";
  import DateInput from "../common/DateInput.svelte";
  import Input from "../common/Input.svelte";
  import Label from "../common/Label.svelte";
  import NumberInput from "../common/NumberInput.svelte";
  import ColorSelector from "./ColorSelector.svelte";

  let title = "";
  let startDate = "";
  let startHour = 0;
  let startMinute = 0;

  let color: Color = "#";
  let endDate = "";
  let endHour = 0;
  let endMinute = 0;

  const onSubmit = () => {
    alert("Clicked!");
    const startDatetime = dateToDatetimeISOString(
      startDate,
      startHour,
      startMinute
    );
    const endDatetime = dateToDatetimeISOString(endDate, endHour, endMinute);
    const task: TaskCreate = {
      body: title,
      start_at: startDatetime,
      end_at: endDatetime,
      colors: color,
    };
    create_task(task)
      .then(() => {
        alert("Done!");
      })
      .catch((e) => {
        alert(e);
      });
  };
</script>

<div class="modal">
  <h2>Create a Task</h2>
  <div>
    <Label forForm="title" value="Title" />
    <Input bind:value={title} />
  </div>
  <div>
    <Label forForm="start-date" value="Start Date" />
    <DateInput bind:value={startDate} />
    <span class="time-input-container">
      <NumberInput bind:value={startHour} min={"0"} max={"23"} />
    </span>
    <span class="time-input-container">
      <NumberInput bind:value={startMinute} min={"0"} max={"23"} />
    </span>
  </div>
  <div>
    <Label forForm="end-date" value="End Date" />
    <DateInput bind:value={endDate} />
    <span class="time-input-container">
      <NumberInput bind:value={endHour} min={"0"} max={"23"} />
    </span>
    <span class="time-input-container">
      <NumberInput bind:value={endMinute} min={"0"} max={"23"} />
    </span>
  </div>
  <div>
    <Label value="Color" forForm="color" />
    <ColorSelector bind:value={color} />
  </div>
  <div class="button-container">
    <Button onClick={onSubmit}>Create</Button>
  </div>
  {startDate}
</div>

<style>
  h2 {
    margin: 10px 0 40px 0;
  }
  .time-input-container {
    margin-left: 20px;
  }
  .button-container {
    margin: 20px 0;
    display: flex;
    justify-content: center;
  }
  .modal {
    max-width: 800px;
    width: 100%;
    padding: 30px;
    background-color: #404040;
    border-radius: 24px;
  }
  .colon {
    margin-inline: 5px;
  }
</style>
