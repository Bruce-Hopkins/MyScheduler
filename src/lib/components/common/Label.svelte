<script lang="ts">
  export let value: string;
  export let forForm: string;
</script>

<label for={forForm}>{value}</label>

<style>
  label {
    display: block;
    margin: 20px 0;
  }
</style>
