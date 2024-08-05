<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { fly, fade, slide } from "svelte/transition";
  import ContentCopy from "svelte-material-icons/ContentCopy.svelte";
  import EyeOffOutline from "svelte-material-icons/EyeOffOutline.svelte";
  import EyeOutline from "svelte-material-icons/EyeOutline.svelte";
  import TrashCanOutline from "svelte-material-icons/TrashCanOutline.svelte";
  import TextLong from "svelte-material-icons/TextLong.svelte";
  import KeyVariant from "svelte-material-icons/KeyVariant.svelte";
  import AsteriskCircleOutline from "svelte-material-icons/AsteriskCircleOutline.svelte";
  import FileDocumentAlertOutline from "svelte-material-icons/FileDocumentAlertOutline.svelte";

  interface Secret {
    label: string;
    kind: string;
    data?: string;
    showButtons?: boolean;
  }

  export let secrets: Secret[] = [];

  async function getLabels() {
    secrets = await invoke<Secret[]>("get_labels");
    secrets = secrets.map((label) => ({ ...label, showButtons: false }));
  }

  async function showData(label: string) {
    let secret = secrets.find((x) => x.label === label);
    if (secret) {
      secret.data = "decrypting...";
      secret.data = await invoke<string | undefined>("get_secret", {
        label: label,
      });
      secrets = secrets;
    }
  }

  function hideData(label: string) {
    let secret = secrets.find((x) => x.label === label);
    if (secret) {
      secret.data = undefined;
      secrets = secrets;
    }
  }

  function setHoverState(label: Secret, state: boolean) {
    label.showButtons = state;
    secrets = secrets;
  }

  async function handleDelete(label: string) {
    await invoke("delete_secret", { label: label });
    await getLabels();
  }
</script>

<div
  class="flex flex-col w-full content-center p-4"
  in:fade={{ duration: 150, delay: 175 }}
  out:fade={{ duration: 150 }}
>
  <div class="flex justify-end w-100">
    <button on:click={() => goto("/new")}>New</button>
  </div>
  <h1 class="mb-4 text-3xl text-center">Secrets</h1>
  <div class="flex flex-col gap-3">
    {#each secrets as secret (secret.label)}
      <div
        class="list-item p-2"
        on:mouseenter={() => setHoverState(secret, true)}
        on:mouseleave={() => setHoverState(secret, false)}
        in:fly|global={{ y: 300, duration: 200, delay: 175 }}
        out:fade|global={{ duration: 150 }}
        role="region"
      >
        <div class="item-icon">
          {#if secret.kind === "password"}
            <KeyVariant />
          {:else if secret.kind === "text"}
            <TextLong />
          {:else}
            <FileDocumentAlertOutline />
          {/if}
        </div>
        <div class="item-label">
          {secret.label}
        </div>
        <div class="item-secret">
          {#if secret.data !== undefined}
            <p>{secret.data}</p>
          {:else}
            {#each [...Array(6).keys()] as _}
              <AsteriskCircleOutline size="0.9rem" />
            {/each}
          {/if}
        </div>
        <div class="buttons-container">
          {#if secret.showButtons}
            <div
              class="item-buttons"
              transition:fly={{ x: 300, duration: 300 }}
            >
              <button class="item-button"><ContentCopy /></button>
              {#if secret.data !== undefined}
                <button
                  class="item-button"
                  on:click={() => hideData(secret.label)}><EyeOutline /></button
                >
              {:else}
                <button
                  class="item-button"
                  on:click={async () => showData(secret.label)}
                  ><EyeOffOutline /></button
                >
              {/if}
              <button
                class="item-button"
                on:click={async () => handleDelete(secret.label)}
                ><TrashCanOutline /></button
              >
            </div>
          {/if}
        </div>
      </div>
    {/each}
  </div>
</div>

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
    justify-content: center;
    padding-right: 2rem;
    align-content: center;
    margin-top: auto;
    margin-bottom: auto;
  }

  .item-secret {
    grid-area: secret;
    display: flex;
    justify-content: center;
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
    left: 2rem;
    transform: translateY(-50%);
  }

  .item-button {
    border: none;
  }
</style>
