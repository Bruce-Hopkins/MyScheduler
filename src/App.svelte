<script lang="ts">
  import svelteLogo from "./assets/svelte.svg";
  import viteLogo from "/vite.svg";
  import Counter from "./lib/Counter.svelte";
  import type { TaskCreate, TaskRes } from "./lib/types/tasks";
  import { get_all_tasks } from "./lib/api/tasks-api";
  import Task from "./lib/components/Task.svelte";
  import Schedule from "./lib/components/Schedule.svelte";
  import Taskgroup from "./lib/components/Taskgroup.svelte";
  // When using the Tauri global script (if not using the npm package)

  let tasks: TaskRes[][] = [];
  async function get_tasks() {
    tasks = await get_all_tasks();
  }
  get_tasks();

  //   console.log(result);
  // }
  // create_task();

  // console.log(tasks);
</script>

<main>
  <h1>Hey</h1>

  <Schedule>
    {#each tasks as task}
      <Taskgroup tasks={task} />
    {/each}
  </Schedule>
  <div>
    <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
      <img src={viteLogo} class="logo" alt="Vite Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank" rel="noreferrer">
      <img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>
  <h1>Vite + Svelte</h1>

  <div class="card">
    <Counter />
  </div>

  <p>
    Check out <a
      href="https://github.com/sveltejs/kit#readme"
      target="_blank"
      rel="noreferrer">SvelteKit</a
    >, the official Svelte app framework powered by Vite!
  </p>

  <p class="read-the-docs">Click on the Vite and Svelte logos to learn more</p>
</main>

<style>
  main {
    width: 90vw;
  }
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .read-the-docs {
    color: #888;
  }
</style>
