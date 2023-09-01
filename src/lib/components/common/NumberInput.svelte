<script lang="ts">
  type numberString = `${number}`;
  export let value: number;
  export let min: numberString | undefined = undefined;
  export let max: numberString | undefined = undefined;
  export let name: string | undefined = undefined;
  export let step: numberString | undefined = undefined;
</script>

<input {name} type="number" bind:value {min} {max} {step} />

<style>
  input {
    border: #8f8f8f 1px solid;
    padding: 10px;
  }
</style>
