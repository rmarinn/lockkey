<script lang="ts">
  import Icon from "@iconify/svelte";

  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  import type { Response } from "@types";
  import ListItem from "./ListItem.svelte";
  import FilterButton from "./FilterButton.svelte";

  interface Secret {
    label: string;
    kind: string;
  }

  let secrets: Secret[] = [];
  let showPasswords: boolean = true;
  let showText: boolean = true;

  $: filteredSecrets = secrets.filter((s) => {
    return (
      (s.kind === "password" && showPasswords) ||
      (s.kind === "text" && showText)
    );
  });

  async function getSecrets() {
    let resp = await invoke<Response<Secret[]>>("get_labels");

    if (resp.success && resp.body !== undefined) {
      secrets = resp.body;
    }
  }

  async function logOut() {
    if (await invoke<boolean>("logout")) {
      goto("/login");
    }
  }

  async function onSecretDeleted(event: CustomEvent<{ label: string }>) {
    const idx = secrets.findIndex((s) => s.label === event.detail.label);
    if (idx !== -1) {
      secrets.splice(idx, 1);
      secrets = secrets;
    }
  }

  onMount(async () => {
    await getSecrets();
  });
</script>

<nav class="navbar">
  <button class="nav-btn" on:click={() => goto("new_secret")}
    ><Icon icon="mdi:plus-circle-outline" width="32px" height="32px" /></button
  >
  <button class="nav-btn" on:click={logOut} aria-label="Log out"
    ><Icon icon="mdi:logout-variant" width="32px" height="32px" /></button
  >
</nav>

<div class="flex flex-col flex-grow gap-[24px] w-full items-center">
  <div class="flex justify-between min-w-[600px] items-center">
    <h1 class="title text-xl">Secrets</h1>
    <div class="flex gap-[24px]">
      <FilterButton
        toggled={showPasswords}
        on:clicked={() => {
          showPasswords = !showPasswords;
        }}
        ariaLabel="Toggle password filter"
      >
        <Icon icon="mdi:key-variant" width="24px" height="24px" />
      </FilterButton>
      <FilterButton
        toggled={showText}
        on:clicked={() => {
          showText = !showText;
        }}
        ariaLabel="Toggle text secret filter"
      >
        <Icon icon="mdi:text-long" width="24px" height="24px" />
      </FilterButton>
    </div>
  </div>

  {#if secrets.length === 0}
    <div
      class="flex flex-col flex-grow w-full justify-center text-center pb-24"
      in:fade={{ duration: 300 }}
    >
      <h1>You have no secrets yet.</h1>
    </div>
  {:else if filteredSecrets.length === 0}
    <div
      class="flex flex-col flex-grow w-full justify-center text-center pb-24"
      in:fade={{ duration: 300 }}
    >
      <h1>All filters are off.</h1>
    </div>
  {:else}
    {#each filteredSecrets as secret (secret.label)}
      <ListItem
        label={secret.label}
        kind={secret.kind}
        on:secretDeleted={onSecretDeleted}
      />
    {/each}
  {/if}
</div>
