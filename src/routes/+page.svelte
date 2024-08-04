<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  interface Label {
    label: string;
    kind: string;
  }

  let labels: Label[] = [];

  // call the rust function to fetch the labels
  async function getLabels() {
    labels = await invoke<Label[]>("get_labels");
  }

  onMount(() => {
    getLabels();
  });

  async function handleDelete(label: string) {
    await invoke("delete_secret", { label: label });
    await getLabels();
  }
</script>

{#if labels.length === 0}
  <div class="justify-center content-center w-full m-4">
    <div class="flex flex-col content-center gap-4">
      <p class="block m-auto text-3xl">no secrets yet</p>
      <button class="block m-auto" on:click={() => goto("/new")}
        >Create a new secret</button
      >
    </div>
  </div>
{:else}
  <div class="flex flex-col w-full content-center m-4">
    <div class="flex justify-end w-100">
      <button on:click={() => goto("/new")}>New</button>
    </div>
    <h1 class="mb-4 text-3xl text-center">Secrets:</h1>
    <div class="flex flex-col gap-2">
      {#each labels as label}
        <div class="flex justify-center content-center gap-1">
          <p class="mr-3 my-auto">{label.label} ({label.kind})</p>
          <button class="mr-1">Copy</button>
          <button class="mr-1">View</button>
          <button on:click={async () => handleDelete(label.label)}
            >Delete</button
          >
        </div>
      {/each}
    </div>
  </div>
{/if}
