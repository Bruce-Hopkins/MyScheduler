<script>
  import { Link } from "svelte-routing";
  import ClockIcon from "../common/icons/ClockIcon.svelte";
  import FolderIcon from "../common/icons/FolderIcon.svelte";
</script>

<nav class="sidebar">
  <Link to="/schedules">
    <ClockIcon size={18} />
    <span> Schedules </span>
  </Link>
  <Link to="/tasks">
    <FolderIcon size={18} />
    <span> Tasks </span>
  </Link>
</nav>

<style>
  .sidebar {
    padding-top: 20px;
    width: 129px;
    flex-basis: 129px;
    background-color: #404040;

    display: flex;
    justify-content: start;
    flex-direction: column;
  }
  a {
    display: flex;
    flex-direction: row;
    padding: 20px 30px;
    color: white;
    text-decoration: none;
  }
</style>
