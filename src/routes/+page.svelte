<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { Circle2 } from "svelte-loading-spinners";
  import { fade } from "svelte/transition";

  let isAuthenticated: boolean = false;

  onMount(async () => {
    isAuthenticated = await invoke<boolean>("is_authenticated");
    if (isAuthenticated) {
      goto("/secrets");
    } else {
      goto("/new_account");
    }
  });
</script>

<div
  class="w-full h-full flex justify-center"
  in:fade={{ delay: 175, duration: 150 }}
  out:fade={{ duration: 150 }}
>
  <div class="my-auto">
    <Circle2
      size="60"
      colorOuter="#f6f6f6"
      colorInner="#f6f6f6"
      colorCenter="#f6f6f6"
      unit="px"
    />
  </div>
</div>
