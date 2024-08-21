<script lang="ts">
  export let checked: boolean = false;
  export let ariaLabel: string = "";
</script>

<button
  on:click={() => (checked = !checked)}
  class={checked ? "checked" : "unchecked"}
  aria-label={ariaLabel}><slot /></button
>

<style lang="scss">
  @import "@assets/scss/variables";

  button {
    border: solid 0.125rem $primary;
    padding: 0.5rem;
    border-radius: 4rem;
    transition:
      background-color 0.3s ease-out,
      color 0.3s ease-out;
  }

  .checked {
    background-color: $primary;
  }

  .unchecked {
    color: $primary;
    background-color: transparent;
    border-style: dashed;
  }
</style>
