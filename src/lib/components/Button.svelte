<script lang="ts">
  import type { Color, Size } from "../types/common";

  export let backgroundColor: Color = "#71AD87";
  export let color: Color = "#ffffff";
  export let height: Size = "42px";
  export let width: Size = "111px";
  export let fontSize: Size = "16px";
  export let onClick: () => void;
</script>

<button
  on:click={onClick}
  style="color: {color}; background-color: {backgroundColor}; height: {height}; width: {width}; font-size: {fontSize}px"
>
  <slot />
</button>

<style>
  .button {
    border-radius: 7px;
    font-weight: 500;
  }
</style>
