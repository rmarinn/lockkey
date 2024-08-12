<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let toggled: boolean = true;
  export let ariaLabel: string = "Toggle button";
  $: btnClass = toggled ? "filter-toggle-on" : "filter-toggle-off";

  function clickedHandler() {
    dispatch("clicked");
  }
</script>

<button class={btnClass} aria-label={ariaLabel} on:click={clickedHandler}>
  <slot />
</button>

<style lang="scss">
  $text-light: #e9edf1;
  $text-dark: #090e15;
  $background: #90b1da;
  $primary: #1d4a81;
  $secondary: #3e89e5;

  .filter-toggle-on {
    color: $primary;
    border: solid 2px $primary;
    padding: 12px;
    border-radius: 100px;
    display: inline-block;
    transition: transform 0.3s ease-out;

    :global(*) {
      transition:
        transform 0.15s ease-out,
        filter 0.3s ease-out;
      transform-origin: center;
      overflow: visible;
    }

    &:hover {
      border: dashed 2px $primary;
      transform: rotateZ(30deg);

      :global(*) {
        transform: rotateZ(-15deg);
      }
    }
  }

  .filter-toggle-off {
    color: $background;
    border: dashed 2px $primary;
    padding: 12px;
    border-radius: 100px;
    transition: transform 0.3s ease-out;

    :global(*) {
      transition:
        transform 0.15s ease-out,
        filter 0.3s ease-out;
      transform-origin: center;
      filter: drop-shadow(0px 0px 1px $primary);
      overflow: visible;
    }

    &:hover {
      border: solid 2px $primary;
      transform: rotate(30deg);

      :global(*) {
        transform: rotateZ(-15deg);
      }
    }
  }
</style>
