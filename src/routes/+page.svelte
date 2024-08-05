<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
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
    showButtons?: boolean;
  }

  let labels: Label[] = [];

  async function getLabels() {
    labels = await invoke<Label[]>("get_labels");
    labels = labels.map((label) => ({ ...label, showButtons: false }));
  }

  function setHoverState(label: Label, state: boolean) {
    label.showButtons = state;
    labels = labels;
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
      {#each labels as label (label.label)}
        <div
          class="list-item p-2"
          transition:fly={{ y: 200, duration: 300 }}
          on:mouseenter={() => setHoverState(label, true)}
          on:mouseleave={() => setHoverState(label, false)}
          role="region"
        >
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
          <div class="item-secret"><p>******</p></div>
          <div class="buttons-container">
            {#if label.showButtons}
              <div
                class="item-buttons"
                transition:fly={{ x: 300, duration: 300 }}
              >
                <button class="item-button"><ContentCopy /></button>
                <button class="item-button"><EyeOffOutline /></button>
                <button
                  class="item-button"
                  on:click={async () => handleDelete(label.label)}
                  ><TrashCanOutline /></button
                >
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .list-item {
    display: grid;
    grid-template-areas: "icon label secret buttons";
    grid-template-columns: 1fr 0.5fr 0.5fr 1fr;
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
    justify-content: right;
    padding-right: 2rem;
    align-content: center;
    margin-top: auto;
    margin-bottom: auto;
  }

  .item-secret {
    grid-area: secret;
    display: flex;
    justify-content: left;
    padding-left: 2rem;
    align-content: center;
    margin-top: auto;
    margin-bottom: auto;
  }

  .buttons-container {
    grid-area: buttons;
    position: relative;
    overflow: hidden;
    padding: 1.2rem;
  }

  .item-buttons {
    display: flex;
    gap: 0.3rem;
    align-items: center;
    position: absolute;
    top: 50%;
    left: 0.1rem;
    transform: translateY(-50%);
  }

  .item-button {
    border: none;
  }
</style>
