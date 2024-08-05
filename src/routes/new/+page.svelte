<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";

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

<div class="flex flex-col w-full justify-between p-4">
  <div class="flex justify-end">
    <button on:click={() => goto("/")}>Back</button>
  </div>

  <form
    on:submit|preventDefault={handleSubmit}
    class="flex flex-col justify-center content-center gap-4"
  >
    <div class="flex gap-3 mx-auto">
      <label for="input-type" class="mr-2">Secret type:</label>

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

    <div class="flex flex-col gap-2">
      <div
        class={`flex + ${selectedType === "password" ? " justify-center" : ""}`}
      >
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
        <div class="flex justify-center">
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
        ></textarea>
      {:else}
        <p>Unknown type</p>
      {/if}
    </div>

    <div class="flex justify-center">
      <button style="width: 200px;">Save</button>
    </div>
  </form>

  <div></div>
</div>
