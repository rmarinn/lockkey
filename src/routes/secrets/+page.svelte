<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount, type DispatchOptions } from "svelte";
  import { fade } from "svelte/transition";
  import ListItems from "./ListItems.svelte";

  interface Secret {
    label: string;
    kind: string;
    data?: string;
    showButtons?: boolean;
  }

  let secrets: Secret[] = [];

  async function getLabels() {
    secrets = await invoke<Secret[]>("get_labels");
    secrets = secrets.map((label) => ({ ...label, showButtons: false }));
  }

  async function logOut() {
    if (await invoke<boolean>("logout")) {
      goto("/login");
    }
  }

  function handleListUpdated(event: CustomEvent) {
    secrets = event.detail.secrets;
  }

  onMount(async () => {
    await getLabels();
  });

  $: {
    console.log(secrets.length);
  }
</script>

{#if secrets.length === 0}
  <div
    class="grid grid-cols-1 grid-rows-3 auto-rows-fr p-4 h-full"
    in:fade={{ duration: 150, delay: 175 }}
    out:fade={{ duration: 150 }}
  >
    <div class="row-span-1 flex justify-end">
      <button class="mb-auto" on:click={logOut}>Logout</button>
    </div>
    <div class="row-span-1 flex flex-col gap-4 mx-auto justify-center">
      <p class="text-3xl">no secrets yet</p>
      <button on:click={() => goto("/new_secret")}>Create a new secret</button>
    </div>
  </div>
{:else}
  <div
    class="flex flex-col w-full content-center p-4"
    in:fade={{ duration: 150, delay: 175 }}
    out:fade={{ duration: 150 }}
  >
    <div class="row-span-1 mb-auto flex justify-end w-full gap-2">
      <button on:click={logOut}>Logout</button>
      <button on:click={() => goto("/new_secret")}>New</button>
    </div>
    <ListItems {secrets} on:listUpdated={handleListUpdated} />
  </div>
{/if}
