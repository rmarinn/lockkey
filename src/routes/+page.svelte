<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Response } from "@assets/ts/types";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import Loader from "@assets/components/Loader.svelte";

  let isAuthenticated: boolean = false;

  onMount(async () => {
    let resp = await invoke<Response<boolean>>("is_authenticated");
    isAuthenticated = resp.body ?? false;
    if (resp.success && isAuthenticated) {
      goto("/secrets");
    } else {
      goto("/login");
    }
  });
</script>

<div
  class="w-full flex-grow flex justify-center items-center"
  in:fade={{ delay: 175, duration: 150 }}
  out:fade={{ duration: 150 }}
>
  <Loader />
</div>
