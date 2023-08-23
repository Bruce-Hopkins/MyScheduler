<script lang="ts">
  import { getTimeFromDate } from "../../common/calculations";

  let time = getTimeFromDate(new Date())
  setInterval(changeTime, 1000 )
  function changeTime() {
    time = getTimeFromDate(new Date)
  }
</script>

<div class="topnav">
  <h2>{time}</h2>
</div>
<div class="topnav-slot"/>


<style>
  .topnav {
    width: 100%;
    height: 61px;
    background-color: #404040;
    display: flex;
    justify-content: center;
    align-items: center;
    border-bottom: #8F8F8F solid 1px;
    position: fixed;
  }
  .topnav-slot {
    width: 100%;
    height: 62px;
  }
  
</style>
