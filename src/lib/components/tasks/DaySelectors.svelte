<script lang="ts">
    import DayButton from "./DayButton.svelte";

  const week = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
</script>

<div>
  {#each week as day, index}
    <DayButton name={day} date={new Date()}/>
  {/each}
</div>


