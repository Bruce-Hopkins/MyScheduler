<script lang="ts">
  import type { Color } from "../../types/common";
  import DateInput from "../common/DateInput.svelte";
  import Input from "../common/Input.svelte";
  import Label from "../common/Label.svelte";
  import NumberInput from "../common/NumberInput.svelte";
  import ColorSelector from "./ColorSelector.svelte";

  let title = "";
  let startDate = "";
  let hour = 0;
  let minute = 0;
  let color: Color = "#";
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
  </div>
  <div>
    <Label forForm="time" value="Time" />
    <NumberInput name="hour" bind:value={hour} max="23" min="0" />
    <span class="colon">:</span>
    <NumberInput name="minute" bind:value={minute} step="10" max="59" min="0" />
  </div>
  <div>
    <Label value="Color" forForm="color" />
    <ColorSelector bind:value={color} />
    {color}
  </div>
</div>

<style>
  h2 {
    margin: 10px 0 40px 0;
  }
  .modal {
    padding: 30px;
    background-color: #404040;
    border-radius: 24px;
  }
  .colon {
    margin-inline: 5px;
  }
</style>
