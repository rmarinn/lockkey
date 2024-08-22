<script lang="ts">
  import { MsgType, popupMsgs } from "@ts/popupMsgStore";
  import { fly, fade } from "svelte/transition";
  import { cubicOut } from "svelte/easing";
  import { flip } from "svelte/animate";

  function getMsgClass(msgType: MsgType) {
    let msgClass: string = "popup-msg";
    switch (msgType) {
      case MsgType.Success:
        msgClass += " msg-success";
        break;
      case MsgType.Warning:
        msgClass += " msg-warning";
        break;
      case MsgType.Error:
        msgClass += " msg-error";
        break;
      default:
        break;
    }
    return msgClass;
  }
</script>

{#if $popupMsgs.length > 0}
  <div class="popup-overlay">
    {#each $popupMsgs as msg (msg.id)}
      <div
        class={getMsgClass(msg.type)}
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
  @import "@assets/scss/variables";

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

  .msg-warning {
    background: darken($warning, 10);
    color: $text-dark;
  }
</style>
