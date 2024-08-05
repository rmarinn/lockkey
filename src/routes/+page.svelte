<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import ListItems from "./ListItems.svelte";

  interface Secret {
    label: string;
    kind: string;
    data?: string;
    showButtons?: boolean;
  }

  let secrets: Secret[] = [];
  let ready: boolean = false;
  let count = 0;

  async function getLabels() {
    secrets = await invoke<Secret[]>("get_labels");
    secrets = secrets.map((label) => ({ ...label, showButtons: false }));
  }

  onMount(async () => {
    await getLabels();
    ready = true;
    count += 1;
  });
</script>

{#if secrets.length === 0}
  <div class="flex flex-col justify-center content-center gap-4 h-full">
    <p class="text-3xl mx-auto">no secrets yet</p>
    <button class="mx-auto" on:click={() => goto("/new")}
      >Create a new secret</button
    >
  </div>
{:else}
  <ListItems {secrets} />
{/if}
