<script lang="ts">
  import { onNavigate } from "$app/navigation";
  import "../assets/scss/styles.scss";
  import PopupMsgs from "./PopupMsgOverlay.svelte";
  import SessionHandler from "./SessionHandler.svelte";
  import Alert from "./AlertOverlay.svelte";

  onNavigate((navigation) => {
    // @ts-ignore
    if (!document.startViewTransition) return;

    return new Promise((resolve) => {
      // @ts-ignore
      document.startViewTransition(async () => {
        resolve();
        await navigation.complete;
      });
    });
  });
</script>

<Alert />
<PopupMsgs />
<SessionHandler />

<div class="flex flex-col justify-start items-start min-h-lvh min-w-max">
  <slot />
</div>
