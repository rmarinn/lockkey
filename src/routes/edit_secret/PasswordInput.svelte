<script lang="ts">
  import Icon from "@iconify/svelte";
  import { fly, slide } from "svelte/transition";
  import { cubicIn, cubicOut } from "svelte/easing";
  import CheckBox from "./CheckBox.svelte";
  import Slider from "./Slider.svelte";
  import { userPrefs } from "@ts/userPrefs";
  import { onMount, onDestroy } from "svelte";

  $: passwdPrefs = $userPrefs.passwdGen;

  onMount(() => {
    excludedCharsInput = passwdPrefs.excludedChars;
  });

  onDestroy(() => {
    $userPrefs.save();
  });

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

  $: labelIsTooShort = label !== "" && label.length < minLblLen;
  $: labelIsTooLong = label.length > maxLblLen;
  $: passwordIsTooShort = passwd !== "" && passwd.length < minPasswdLen;
  $: passwordIsTooLong = passwd.length > maxPasswdLen;

  function togglePasswdVisibility() {
    showPasswd = !showPasswd;
    passwdInput.type = showPasswd ? "text" : "password";
  }

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
    if (passwdPrefs.useLetters) charSet += lowercaseLetters;
    if (passwdPrefs.useUppercase) charSet += uppercaseLetters;
    if (passwdPrefs.useNumbers) charSet += numbers;
    if (passwdPrefs.useSymbols) charSet += symbols;
    if (charSet === "") return "";

    const excludedChars = new Set(passwdPrefs.excludedChars);
    charSet = charSet
      .split("")
      .filter((char) => !excludedChars.has(char))
      .join("");
    charSet = shuffleString(charSet);

    let passwd = "";

    for (let i = 0; i < passwdPrefs.passwdLength; i++) {
      passwd += charSet[Math.floor(Math.random() * charSet.length)];
    }

    return passwd;
  }

  function processExcludedChars() {
    let filtered: Set<string> = new Set(excludedCharsInput);
    excludedCharsInput = Array.from(filtered).join("");
    passwdPrefs.excludedChars = excludedCharsInput;
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
  class="flex flex-col gap-4 mx-auto items-center"
  in:fly={{ x: 300, duration: 150, delay: 150 }}
>
  <div class="card flex flex-col gap-4">
    <div class="relative mx-auto">
      <div class="lbl-icon">
        <Icon icon="mdi:key-variant" />
      </div>
      <div class="flex flex-col gap-1 w-[250px] text-nowrap overflow-visible">
        <input
          type="text"
          name="password-label-input"
          placeholder="label"
          class="p-1 flex-grow h-10"
          bind:value={label}
          autocomplete="off"
          aria-label="Label for the secret"
          min={minLblLen}
          max={maxLblLen}
        />
        <div class="flex flex-col items-center gap-0.5">
          {#if labelIsTooShort}
            <div
              class="italic text-red-900 text-sm"
              transition:slide={{ duration: 300, axis: "y", easing: cubicOut }}
            >
              Label must be longer than {minLblLen} characters
            </div>
          {:else if labelIsTooLong}
            <div
              class="italic text-red-900 text-sm"
              transition:slide={{ duration: 300, axis: "y", easing: cubicOut }}
            >
              Label cannot be longer than {maxLblLen} characters
            </div>
          {/if}
          {#if labelIsTooShort || labelIsTooLong}
            <div
              class="italic text-red-900 text-sm"
              transition:slide={{ duration: 300, axis: "y", easing: cubicOut }}
            >
              Label character count: {label.length}
            </div>
          {/if}
        </div>
      </div>
    </div>

    <div class="relative mx-auto">
      <button
        class="passwd-vis-toggle"
        on:click|preventDefault={togglePasswdVisibility}
      >
        {#if showPasswd}
          <Icon icon="mdi:eye-outline" />
        {:else}
          <Icon icon="mdi:eye-off-outline" />
        {/if}
      </button>

      <div class="flex flex-col gap-2 w-[250px] text-nowrap overflow-visible">
        <input
          type="password"
          name="password-input"
          id="password-input"
          placeholder="password"
          class="p-1 h-10"
          bind:value={passwd}
          autocomplete="off"
          aria-label="Password"
          bind:this={passwdInput}
        />
        <div class="flex flex-col items-center gap-2">
          {#if passwordIsTooShort}
            <div
              class="italic text-red-900 text-sm"
              transition:slide={{ duration: 300, axis: "y", easing: cubicOut }}
            >
              Password must be longer than {minPasswdLen} characters
            </div>
          {:else if passwordIsTooLong}
            <div
              class="italic text-red-900 text-sm"
              transition:slide={{ duration: 300, axis: "y", easing: cubicOut }}
            >
              Password cannot be longer than {maxPasswdLen} characters
            </div>
          {/if}
          {#if passwordIsTooShort || passwordIsTooShort}
            <div
              class="italic text-red-900 text-sm"
              transition:slide={{ duration: 300, axis: "y", easing: cubicOut }}
            >
              Password character count: {passwd.length}
            </div>
          {/if}
        </div>
      </div>

      <div class="passwd-opts">
        <button
          class="input-btn"
          on:click|preventDefault={copyToClipboard}
          aria-label="Copy to clipboard"
        >
          <Icon icon="mdi:content-copy" />
        </button>
        <button
          class="passwd-opt"
          on:click|preventDefault={() => {
            passwd = generateRandomPasswd();
          }}
          aria-label="Randomize password"
        >
          <Icon icon="mdi:dice-5-outline" />
        </button>
        <button
          class="passwd-opt"
          on:click|preventDefault={() => {
            showPasswdSettings = !showPasswdSettings;
          }}
          aria-label="Show password randomization settings"
        >
          <Icon icon="mdi:cog-outline" />
        </button>
      </div>
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
          bind:value={passwdPrefs.passwdLength}
          ariaLabel="Password length slider"
        />
      </div>
      <div class="flex items-center gap-8">
        <div class="italic">toggle</div>
        <div class="flex-grow flex justify-end gap-2">
          <CheckBox
            bind:checked={passwdPrefs.useLetters}
            ariaLabel="Toggle letters">letters</CheckBox
          >
          <CheckBox
            bind:checked={passwdPrefs.useNumbers}
            ariaLabel="Toggle numbers">numbers</CheckBox
          >
          <CheckBox
            bind:checked={passwdPrefs.useSymbols}
            ariaLabel="Toggle symbols">symbols</CheckBox
          >
          <CheckBox
            bind:checked={passwdPrefs.useUppercase}
            ariaLabel="Toggle uppercase">uppercase</CheckBox
          >
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
          aria-label="Excluded characters"
        />
      </div>
    </div>
  {/if}
</div>

<style lang="scss">
  @import "@assets/scss/variables";

  .card {
    padding: 1.5rem;
    gap: 0.5rem;
    min-width: 32rem;
  }

  .lbl-icon {
    position: absolute;
    left: -2rem;
    top: 1.25rem;
    transform: translateY(-50%);
  }

  .passwd-vis-toggle {
    position: absolute;
    left: -2rem;
    top: 1.25rem;
    transform: translateY(-50%);
  }

  .passwd-opts {
    position: absolute;
    right: -1rem;
    top: 50%;
    transform: translateY(-50%) translateX(100%);

    display: flex;
    justify-content: end;
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
