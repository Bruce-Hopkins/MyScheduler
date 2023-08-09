<!-- App.svelte -->
<script>
  import { Router, Route } from "svelte-routing";
  import Overview from "./lib/pages/Overview.svelte";
  import Tasks from "./lib/pages/Tasks.svelte";
  import Schedules from "./lib/pages/Schedules.svelte";

  export let url = "/";
</script>

<Router {url}>
  <Route path="/" component={Overview} />
  <Route path="/tasks" component={Tasks} />
  <Route path="/schedules" component={Schedules} />
</Router>
