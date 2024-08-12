<script lang="ts">
  import { onNavigate } from "$app/navigation";

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

<div class="flex flex-col justify-start items-start p-8 min-h-lvh min-w-max">
  <slot />
</div>
