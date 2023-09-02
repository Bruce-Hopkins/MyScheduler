<script lang="ts">
  import type { Color } from "../../types/common";
  import ColorInput from "./ColorInput.svelte";
  export let value: Color;
</script>

<div>
  <ColorInput bind:selectedColor={value} color="#7D5B82" />
  <ColorInput bind:selectedColor={value} color="#96E6B3" />
  <ColorInput bind:selectedColor={value} color="#569BCC" />
  <div class="example-color" style="background-color: {value};" />
</div>

<style>
  .example-color {
    margin: 20px 0;
    width: 50px;
    height: 50px;
    border-radius: 10px;
  }
</style>
