<script lang="ts">
  import Input from "../common/Input.svelte";
  import Label from "../common/Label.svelte";

  let value = "";
</script>

<div class="modal">
  <h2>Create a Task</h2>
  <Label forForm="title" value="Title" />
  <Input bind:value />
</div>

<style>
  h2 {
    margin: 10px 0 40px 0;
  }
  .modal {
    padding: 30px;
    background-color: #404040;
    border-radius: 24px;
  }
</style>
