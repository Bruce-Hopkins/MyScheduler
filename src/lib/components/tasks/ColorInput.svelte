<script lang="ts">
  import type { Color } from "../../types/common";

  export let color: Color;
  export let selectedColor: Color;

  const onClick = () => {
    selectedColor = color;
  };
</script>

<button style="background-color: {color};" on:click={onClick} />

<style>
  button {
    padding: 10px;
    border-radius: 100%;
    margin-inline: 3px;
  }
</style>
