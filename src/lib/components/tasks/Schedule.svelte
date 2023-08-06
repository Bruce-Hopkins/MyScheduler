<script lang="ts">
  import { pxPerMinute } from "../../common/calculations";

  const hoursOfTheDay = () => {
    let numbers: string[] = [];
    for (let i = 0; i < 24; i++) {
      numbers.push(`${i}:00`);
    }
    return numbers;
  };

  const hours = hoursOfTheDay();
</script>

<div class="schedule">
  <div class="schedule-container">
    <div class="task-slots">
      <slot />
    </div>
    {#each hours as hourSlot}
      <div class="hour-slot" style="margin-bottom: {pxPerMinute(60)}px;">
        <div class="slot-left">
          {hourSlot}
        </div>
        <div class="slot-right" />
      </div>
    {/each}
  </div>
</div>

<style>
  .schedule {
    margin: 24px 0;
    position: relative;
    display: block;
  }
  .schedule-container {
    display: block;
    position: relative;
    display: flex;
    flex-direction: column;
  }
  .hour-slot {
    /* The -24 is to accomidate the size of the numbers */
    margin-top: -24px;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
  }
  .slot-right {
    border-top: 1px solid #6c757d;
    width: 100%;
    z-index: 10;
  }
  .slot-left {
    padding-right: 20px;
  }
  .task-slots {
    display: block;
    position: absolute;
    top: -12px;
    width: calc(100% - 50px);
    right: 0;
  }
</style>
