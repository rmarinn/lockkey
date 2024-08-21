<script lang="ts">
  import Icon from "@iconify/svelte";
  import { alertMsg } from "../assets/ts/alertMsgStore";
  import { fade } from "svelte/transition";
  import { cubicOut, cubicIn } from "svelte/easing";
  import { clearAlertMsg } from "../assets/ts/alertMsgStore";

  let msg: string | null = null;
  $: alertMsg.subscribe((value) => (msg = value));
</script>

{#if msg !== null}
  <div
    class="alert"
    in:fade={{ duration: 300, easing: cubicOut }}
    out:fade={{ duration: 300, easing: cubicIn }}
  >
    <div class="card">
      <button on:click={clearAlertMsg}
        ><Icon icon="mdi:close" width="2rem" height="2rem" /></button
      >
      {msg}
    </div>
  </div>
{/if}

<style lang="scss">
  @import "../assets/scss/variables";

  .alert {
    position: absolute;
    width: 100vw;
    height: 100vh;
    z-index: 10;
    backdrop-filter: blur(2px);
    background-color: transparent;

    display: flex;
    justify-content: center;
    align-items: center;

    > .card {
      position: relative;
      background-color: $background;

      > button {
        position: absolute;
        right: 1rem;
        top: 1rem;
        filter: drop-shadow(1px 1px 2px rgba($primary, 0.5));
        transition: filter 0.3s ease-out;

        &:hover {
          filter: drop-shadow(4px 4px 4px rgba($primary, 0.9));
        }
      }
    }
  }
</style>
