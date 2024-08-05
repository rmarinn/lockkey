<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import ContentCopy from "svelte-material-icons/ContentCopy.svelte";
  import EyeOffOutline from "svelte-material-icons/EyeOffOutline.svelte";
  import EyeOutline from "svelte-material-icons/EyeOutline.svelte";
  import TrashCanOutline from "svelte-material-icons/TrashCanOutline.svelte";
  import TextLong from "svelte-material-icons/TextLong.svelte";
  import KeyVariant from "svelte-material-icons/KeyVariant.svelte";
  import FileDocumentAlertOutline from "svelte-material-icons/FileDocumentAlertOutline.svelte";

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
    <div class="flex flex-col gap-3">
      {#each labels as label}
        <div class="list-item p-2">
          <div>
            <!-- spacer -->
          </div>
          <div class="item-icon">
            {#if label.kind === "password"}
              <KeyVariant />
            {:else if label.kind === "text"}
              <TextLong />
            {:else}
              <FileDocumentAlertOutline />
            {/if}
          </div>
          <div class="item-label">
            {label.label}
          </div>
          <div class="item-buttons">
            <button class="mr-1"><ContentCopy /></button>
            <button class="mr-1"><EyeOffOutline /></button>
            <button on:click={async () => handleDelete(label.label)}
              ><TrashCanOutline /></button
            >
          </div>
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .list-item {
    display: grid;
    grid-template-areas: "spacer1 icon label spacer2 buttons";
    grid-template-columns: 1fr 1fr 1fr 1fr 1fr;
    border-radius: 0.5rem;
  }

  .list-item:hover {
    box-shadow: 0px 0px 0px 2px whitesmoke;
  }

  .item-icon {
    grid-area: icon;
    display: flex;
    justify-content: end;
    align-items: center;
  }

  .item-label {
    grid-area: label;
    display: flex;
    justify-content: center;
    align-content: center;
    margin-top: auto;
    margin-bottom: auto;
  }

  .item-buttons {
    grid-area: buttons;
    display: flex;
    justify-content: end;
    align-items: center;
  }
</style>
