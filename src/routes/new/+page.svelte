<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { fly } from "svelte/transition";

  const inputTypes: string[] = ["password", "text"];
  let selectedType: string = "password";

  let label: string = "";
  let data: string = "";

  // clear secret when selectedType changes
  $: {
    selectedType;
    data = "";
  }

  async function handleSubmit() {
    await invoke("new_secret", {
      kind: selectedType,
      label: label,
      data: data,
    });

    goto("/");
  }
</script>

<div class="flex flex-col w-full p-4">
  <div class="flex justify-end">
    <button on:click={() => goto("/")}>Back</button>
  </div>

  <form
    on:submit|preventDefault={handleSubmit}
    class="flex flex-col content-center gap-4 h-full"
  >
    <h1 class="text-3xl text-center">New Secret</h1>
    <div class="flex gap-3 mx-auto">
      <label for="input-type" class="mr-2">Type:</label>

      {#each inputTypes as type}
        <div>
          <input
            type="radio"
            name="input-type"
            id="input-type"
            value={type}
            bind:group={selectedType}
          />
          {type}
        </div>
      {/each}
    </div>

    <div class="flex flex-col gap-3 overflow-hidden p-2">
      <div class="flex justify-center">
        <input
          type="text"
          name="password-label-input"
          placeholder="label"
          class="text-center p-1 text-black"
          bind:value={label}
          autocomplete="off"
        />
      </div>
      {#if selectedType === "password"}
        <div
          class="flex justify-center"
          in:fly={{ x: 300, duration: 150, delay: 150 }}
          out:fly={{ x: -300, duration: 150 }}
        >
          <input
            type="text"
            name="password-input"
            id="password-input"
            placeholder="password"
            class="text-center p-1 text-black"
            bind:value={data}
            autocomplete="off"
          />
        </div>
      {:else if selectedType === "text"}
        <textarea
          name="text-input"
          id="text-input"
          placeholder="place your text here..."
          rows="5"
          class="p-1 text-black"
          bind:value={data}
          autocomplete="off"
          in:fly={{ x: 300, duration: 150, delay: 150 }}
          out:fly={{ x: -300, duration: 150 }}
        ></textarea>
      {:else}
        <p>Unknown type</p>
      {/if}
    </div>

    <div class="flex justify-center justify-self-end mt-auto">
      <button style="width: 200px;">Save</button>
    </div>
  </form>

  <div></div>
</div>
