<script>
  import { Link } from "svelte-routing";
  import ClockIcon from "../common/icons/ClockIcon.svelte";
  import FolderIcon from "../common/icons/FolderIcon.svelte";
  import "./layout.css";
</script>

<div class="sidebar-slot" />
<nav class="sidebar">
  <Link to="/schedules">
    <div class="icon-container">
      <ClockIcon size={20} />
    </div>
    <span> Schedules </span>
  </Link>
  <Link to="/tasks">
    <div class="icon-container">
      <FolderIcon size={20} />
    </div>
    <span> Tasks </span>
  </Link>
</nav>

<style>
  .sidebar-slot {
    width: 170px;
    padding-inline: 12px;
    height: 100%;
    position: relative;
    display: block;
  }
  .sidebar {
    top: 62px;
    padding-top: 20px;
    width: 170px;
    flex-basis: 170px;
    background-color: #404040;

    display: flex;
    justify-content: start;
    flex-direction: column;
    position: fixed;
    height: 100%;
  }
  .icon-container {
    display: flex;
    align-items: center;
    justify-content: start;
  }
  span {
    padding-left: 10px;
    font-weight: 600;
    font-size: 1.05rem;
  }
</style>
