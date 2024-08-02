<script>
  import { goto } from "$app/navigation";
  // import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  /** @type {string[]} **/
  let labels = [];

  // call the rust function to fetch the labels
  // async function getLabels() {
  //   labels = await invoke("get_labels");
  // }

  async function getLabels() {
    labels = ["password1", "anotha one", "and yet anotha one"];
    // labels = [];
  }

  onMount(() => {
    getLabels();
  });
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
        <p class="mr-3">{label}</p>
        <button class="mr-1">Copy</button>
        <button>View</button>
      </div>
    {/each}
  </div>
{/if}
