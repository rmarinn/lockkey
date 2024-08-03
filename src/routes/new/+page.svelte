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

<div class="col w-100" style="justify-content: space-between; margin: 1rem;">
  <div class="row" style="justify-content: end;">
    <button on:click={() => goto("/")}>Back</button>
  </div>

  <form
    on:submit|preventDefault={handleSubmit}
    class="row justify-content-center align-items-center w-100"
    style="flex: 3;"
  >
    <div class="col">
      <div class="row mb-2">
        <label for="input-type" class="mr-2">Secret type:</label>

        {#each inputTypes as type}
          <div class="mr-1">
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

      <div class="mb-2 col">
        <div
          class={`row mb-1 + ${selectedType === "password" ? " justify-content-center" : ""}`}
        >
          <input
            type="text"
            name="password-label-input"
            placeholder="label"
            style="text-align: center;"
            bind:value={label}
            autocomplete="off"
          />
        </div>
        {#if selectedType === "password"}
          <div class="row justify-content-center mb-1">
            <input
              type="text"
              name="password-input"
              id="password-input"
              placeholder="password"
              style="text-align: center;"
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
            cols="100"
            bind:value={data}
            autocomplete="off"
          ></textarea>
        {:else}
          <p>Unknown type</p>
        {/if}
      </div>

      <div class="row justify-content-center">
        <button style="width: 200px;">Save</button>
      </div>
    </div>
  </form>

  <div></div>
</div>
