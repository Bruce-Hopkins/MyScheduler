<script lang="ts">
  import { DaySelector } from "../../../logic/daySelector";
  import DayButton from "./DayButton.svelte";

  const daySelector = new DaySelector();
</script>

<div>
  {#each daySelector.daysOfTheWeek as day, index}
    <DayButton {index} name={day} date={new Date()} />
  {/each}
</div>
