<script lang="ts">
  import Icon from "@iconify/svelte";
  import { fly } from "svelte/transition";
  import { cubicIn, cubicOut } from "svelte/easing";
  import CheckBox from "./CheckBox.svelte";
  import Slider from "./Slider.svelte";

  export let minLblLen = 3;
  export let maxLblLen = 32;
  export let minPasswdLen = 6;
  export let maxPasswdLen = 24;
  export let label: string = "";
  export let passwd: string = "";

  let showPasswd: boolean = false;
  let showPasswdSettings: boolean = false;
  let passwdInput: HTMLInputElement;
  let excludedCharsInput: string = "";

  function togglePasswdVisibility() {
    showPasswd = !showPasswd;
    passwdInput.type = showPasswd ? "text" : "password";
  }

  interface PasswordConfig {
    length: number;
    useLetters: boolean;
    useUppercase: boolean;
    useNumbers: boolean;
    useSymbols: boolean;
    excludedChars: string;
  }

  export let passwdCfg: PasswordConfig = {
    length: 12,
    useLetters: true,
    useUppercase: true,
    useNumbers: true,
    useSymbols: true,
    excludedChars: "",
  };

  function shuffleString(str: string): string {
    const array = str.split("");
    for (let i = array.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [array[i], array[j]] = [array[j], array[i]]; // Swap elements
    }
    return array.join("");
  }

  function generateRandomPasswd(): string {
    const lowercaseLetters = "abcdefghijklmnopqrstuvwxyz";
    const uppercaseLetters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const numbers = "0123456789";
    const symbols = "!@#$%^&*()-_=+[]{}|;:'\",.<>?/\\~`";

    let charSet = "";
    if (passwdCfg.useLetters) charSet += lowercaseLetters;
    if (passwdCfg.useUppercase) charSet += uppercaseLetters;
    if (passwdCfg.useNumbers) charSet += numbers;
    if (passwdCfg.useSymbols) charSet += symbols;
    if (charSet === "") return "";

    const excludedChars = new Set(passwdCfg.excludedChars);
    charSet = charSet
      .split("")
      .filter((char) => !excludedChars.has(char))
      .join("");
    charSet = shuffleString(charSet);

    let passwd = "";

    for (let i = 0; i < passwdCfg.length; i++) {
      passwd += charSet[Math.floor(Math.random() * charSet.length)];
    }

    return passwd;
  }

  function processExcludedChars() {
    let filtered: Set<string> = new Set(excludedCharsInput);
    excludedCharsInput = Array.from(filtered).join("");
    passwdCfg.excludedChars = excludedCharsInput;
  }

  async function copyToClipboard() {
    navigator.clipboard
      .writeText(passwd)
      .then(() => {
        console.log("Text copied to clipboard");
      })
      .catch((err) => {
        console.log("Failed to copy text: ", err);
      });
  }
</script>

<div
  class="flex flex-col gap-4 mx-auto"
  in:fly={{ x: 300, duration: 150, delay: 150 }}
>
  <div class="card grid grid-cols-[0.3fr_1fr_0.3fr] grid-rows-2 gap-4">
    <div class="flex justify-self-end self-center pr-2">
      <Icon icon="mdi:key-variant" />
    </div>
    <input
      type="text"
      name="password-label-input"
      placeholder="label"
      class="p-1 flex-grow"
      bind:value={label}
      autocomplete="off"
      aria-label="Label for the secret"
      min={minLblLen}
      max={maxLblLen}
    />
    <div></div>

    <div class="flex justify-end items-center pr-2">
      <button
        class="input-btn"
        on:click|preventDefault={togglePasswdVisibility}
      >
        {#if showPasswd}
          <Icon icon="mdi:eye-outline" />
        {:else}
          <Icon icon="mdi:eye-off-outline" />
        {/if}
      </button>
    </div>

    <input
      type="password"
      name="password-input"
      id="password-input"
      placeholder="password"
      class="p-1 flex-grow"
      bind:value={passwd}
      autocomplete="off"
      aria-label="Password"
      bind:this={passwdInput}
    />

    <div class="flex justify-self-start gap-4 pl-2">
      <button class="input-btn" on:click|preventDefault={copyToClipboard}>
        <Icon icon="mdi:content-copy" />
      </button>
      <button
        class="passwd-opt"
        on:click|preventDefault={() => {
          passwd = generateRandomPasswd();
        }}
      >
        <Icon icon="mdi:dice-5-outline" />
      </button>
      <button
        class="passwd-opt"
        on:click|preventDefault={() => {
          showPasswdSettings = !showPasswdSettings;
        }}
      >
        <Icon icon="mdi:cog-outline" />
      </button>
    </div>
  </div>

  {#if showPasswdSettings}
    <div
      class="flex flex-col gap-0 card mx-auto"
      in:fly={{ x: 300, duration: 300, easing: cubicOut }}
      out:fly={{ x: 300, duration: 300, easing: cubicIn }}
    >
      <div class="flex items-center gap-8">
        <div class="whitespace-nowrap italic">password length</div>
        <Slider
          min={minPasswdLen}
          max={maxPasswdLen}
          bind:value={passwdCfg.length}
        />
      </div>
      <div class="flex items-center gap-8">
        <div class="italic">toggle</div>
        <div class="flex-grow flex justify-end gap-2">
          <CheckBox bind:checked={passwdCfg.useLetters}>letters</CheckBox>
          <CheckBox bind:checked={passwdCfg.useNumbers}>numbers</CheckBox>
          <CheckBox bind:checked={passwdCfg.useSymbols}>symbols</CheckBox>
          <CheckBox bind:checked={passwdCfg.useUppercase}>uppercase</CheckBox>
        </div>
      </div>
      <div class="flex items-center gap-8">
        <div class="italic">exclude</div>
        <input
          type="text"
          placeholder="excluded characters"
          class="flex-grow"
          bind:value={excludedCharsInput}
          on:input={processExcludedChars}
          spellcheck="false"
          autocomplete="off"
        />
      </div>
    </div>
  {/if}
</div>

<style lang="scss">
  @import "../../assets/scss/variables";

  .card {
    padding: 1.5rem;
    gap: 0.5rem;
    min-width: 32rem;
  }

  .grid {
    display: grid;
    gap: 0.5rem;
  }

  button.passwd-opt {
    transition: transform 0.3s ease-out;
    &:hover {
      transform: translateY(-1px) rotate(30deg);
    }
  }

  button.input-btn {
    transition: transform 0.3s ease-out;
    &:hover {
      transform: translateY(-1px);
    }
  }
</style>
