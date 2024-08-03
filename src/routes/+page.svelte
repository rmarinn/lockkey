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
  <div class="row justify-content-center align-items-center w-100 m-2">
    <div class="col text-align-center">
      <p>no secrets yet.</p>
      <button on:click={() => goto("/new")}>Create a new secret</button>
    </div>
  </div>
{:else}
  <div class="col w-100 align-items-center m-2">
    <div class="row w-100" style="justify-content: end;">
      <button on:click={() => goto("/new")}>New</button>
    </div>
    <h1 class="mb-3">Secrets:</h1>
    {#each labels as label}
      <div class="row mb-1 justify-content-center align-items-center">
        <p class="mr-3">{label.label} ({label.kind})</p>
        <button class="mr-1">Copy</button>
        <button class="mr-1">View</button>
        <button on:click={async () => handleDelete(label.label)}>Delete</button>
      </div>
    {/each}
  </div>
{/if}
