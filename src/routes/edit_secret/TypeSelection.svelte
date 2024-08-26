<script lang="ts">
  import Icon from "@iconify/svelte";
  export let selectedType: string = "password";
</script>

<div class="flex gap-4 items-center">
  <label for="type-selection">Type: </label>
  <div id="type-selection" class="flex gap-4">
    <div class="radio-grp">
      <input
        type="radio"
        name="secret-kind"
        id="kind-passwd"
        value="password"
        class="radio-input"
        bind:group={selectedType}
        aria-label="Select password type secret"
      />
      <label for="kind-passwd" class="radio-label">
        <Icon icon="mdi:key-variant" />
        Password</label
      >
    </div>
    <div class="radio-grp">
      <input
        type="radio"
        name="secret-kind"
        id="kind-text"
        value="text"
        class="radio-input"
        bind:group={selectedType}
        aria-label="Select text type secret"
      />
      <label for="kind-text" class="radio-label">
        <Icon icon="mdi:text-long" /> Text
      </label>
    </div>
  </div>
</div>

<style lang="scss">
  @import "@assets/scss/variables";

  .radio-input {
    display: none;

    &:checked ~ .radio-label {
      color: $text-light;
      background-color: $primary;
    }
  }

  .radio-label {
    border: solid 2px $primary;
    padding: 0.4rem 0.8rem;
    border-radius: 5rem;
    cursor: pointer;
    transition: background-color 0.3s ease-out;
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .radio-grp {
    transition: transform 0.3s ease-out;
    &:hover {
      transform: translateY(-2px);
    }
  }
</style>
