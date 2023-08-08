<script>
  import ClockIcon from "../common/icons/ClockIcon.svelte";
  import FolderIcon from "../common/icons/FolderIcon.svelte";
</script>

<nav class="sidebar">
  <a href="/schedules">
    <ClockIcon size={18} />
    <span> Schedules </span>
  </a>
  <a href="/tasks">
    <FolderIcon size={18} />
    <span> Tasks </span>
  </a>
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
