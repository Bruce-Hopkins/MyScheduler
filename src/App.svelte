<script lang="ts">
  import svelteLogo from "./assets/svelte.svg";
  import viteLogo from "/vite.svg";
  import Counter from "./lib/Counter.svelte";
  import { invoke } from "@tauri-apps/api";
  import type { TaskCreate, TaskRes } from "./lib/types/tasks";
  // When using the Tauri global script (if not using the npm package)
  let tasks: TaskRes[] = [];
  async function get_tasks() {
    // Invoke the command
    tasks = await invoke("app_get_all_tasks");
    console.log("tasks are", tasks);
  }

  // async function create_task() {
  //   const task: TaskCreate = {
  //     body: "do something at a certain time",
  //     start_at: new Date().toISOString(),
  //     end_at: new Date().toISOString(),
  //     colors: "#A3D9FF",
  //   };

  //   const result = await invoke("app_create_task", { task: task });
  //   console.log(result);
  // }
  // create_task();
  get_tasks().catch((e) => {
    console.error(e);
  });
  // console.log(tasks);
</script>

<main>
  <h1>Hey</h1>
  {#each tasks as task}
    <div class="item">
      <div class="name">{task.body}</div>
    </div>
  {/each}
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
