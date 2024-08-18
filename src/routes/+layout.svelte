<script lang="ts">
  import { onNavigate } from "$app/navigation";
  import "../assets/scss/styles.scss";
  import PopupOverlay from "./PopupOverlay.svelte";

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

<PopupOverlay />

<div class="flex flex-col justify-start items-start min-h-lvh min-w-max">
  <slot />
</div>
