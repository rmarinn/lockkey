<script lang="ts">
  import type { PopupMsg } from "../assets/ts/popupMsgStore";
  import { popupMsgs, MsgType } from "../assets/ts/popupMsgStore";
  import { fly, fade } from "svelte/transition";
  import { cubicOut } from "svelte/easing";
  import { flip } from "svelte/animate";

  let msgs: PopupMsg[] = [];
  $: popupMsgs.subscribe((value) => (msgs = value));
</script>

{#if msgs.length > 0}
  <div class="popup-overlay">
    {#each msgs as msg (msg.msg)}
      <div
        class={"popup-msg msg-" + MsgType[msg.type].toLowerCase()}
        in:fly|global={{ y: 50, duration: 300, easing: cubicOut }}
        out:fade|global={{ duration: 300, easing: cubicOut }}
        animate:flip={{ duration: 200 }}
      >
        {msg.msg}
      </div>
    {/each}
  </div>
{/if}

<style lang="scss">
  @import "../assets/scss/variables";

  .popup-overlay {
    position: absolute;
    left: 50%;
    top: 2rem;
    transform: translateX(-50%);
    z-index: 1000;

    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .popup-msg {
    color: $text-light;
    border: solid 2px $text-light;
    border-radius: 1rem;
    padding: 0.5rem 1rem;
    text-align: center;
  }

  .msg-success {
    background: $success;
  }

  .msg-error {
    background: $error;
  }
</style>
