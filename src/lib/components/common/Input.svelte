<script lang="ts">
  export let value: string;
  export let placeholder: string | undefined = undefined;
  export let name: string | undefined = undefined;
</script>

<input {name} type="text" bind:value {placeholder} />

<style>
  input {
    border: #8f8f8f 1px solid;
    padding: 10px;
  }
</style>
