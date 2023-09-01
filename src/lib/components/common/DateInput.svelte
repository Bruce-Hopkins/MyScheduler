<script lang="ts">
  export let value: string;
  export let name: string | undefined = undefined;
</script>

<input {name} type="date" bind:value />

<style>
  input {
    border: #8f8f8f 1px solid;
    padding: 10px;
  }
</style>
