<!-- App.svelte -->
<script>
  import { Router, Link, Route } from "svelte-routing";
  import Overview from "./lib/pages/Overview.svelte";

  export let url = "/";
</script>

<Router {url}>
  <Route path="/" component={Overview} />
  <Route path="/tasks" component={Overview} />
  <Route path="/schedules" component={Overview} />
</Router>
