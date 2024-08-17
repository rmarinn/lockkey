<script lang="ts">
  import { scale } from "svelte/transition";
  import { cubicOut } from "svelte/easing";

  export let min: number = 1;
  export let max: number = 10;
  export let value: number = 5;
  export let id: string = "";
  export let ariaLabel: string = "";
  export let class_: string = "";
  $: sliderClass = class_ + " slider";

  let sliderValue: HTMLSpanElement;
  let slider: HTMLInputElement;

  let sliderPos: string = "0px";
  let isLabelVisible: boolean = false;

  $: if (slider && sliderValue) {
    const percentage = ((value - min) / (max - min)) * 100;
    const sliderWidth = slider.clientWidth;
    const thumbWidth = 18; // Width of the thumb

    // Calculate the position of the label relative to the slider
    const labelPosition =
      (percentage / 100) * (sliderWidth - thumbWidth) + thumbWidth / 2;

    sliderPos = `${labelPosition}px`;
  }
</script>

<div class={sliderClass}>
  <span class="min-val">{min}</span>
  <div class="field">
    {#if isLabelVisible}
      <div
        bind:this={sliderValue}
        class="slider-label"
        style="left: {sliderPos}"
        transition:scale={{ duration: 300, easing: cubicOut }}
      >
        {value}
      </div>
    {/if}
    <input
      {min}
      {max}
      {id}
      type="range"
      bind:value
      aria-label={ariaLabel}
      step="1"
      bind:this={slider}
      on:mouseenter={() => (isLabelVisible = true)}
      on:mouseleave={() => (isLabelVisible = false)}
    />
  </div>
  <span class="max-val">{max}</span>
</div>

<style lang="scss">
  @import "../../assets/scss/variables";
  @import "../../assets/scss/utils";

  .slider {
    display: flex;
    gap: 0.5rem;
    justify-content: space-between;
    align-items: center;
    width: 100%;
  }

  input[type="range"] {
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    height: 5px;
    border-radius: 5px;
    padding: 0;
    display: flex;
    gap: 0.5rem;
    align-items: center;
    width: 100%;
    background-color: rgba($secondary, 0.3);

    &::-webkit-slider-thumb {
      appearance: none;
      -webkit-appearance: none;
      width: 18px;
      height: 18px;
      cursor: ew-resize;
      background-color: $text-light;
      border-radius: 50%;
    }

    &::-moz-range-thumb {
      appearance: none;
      -webkit-appearance: none;
      width: 12px;
      height: 12px;
      cursor: ew-resize;
      background-color: $text-light;
      border-radius: 50%;
    }

    &::-ms-thumb {
      appearance: none;
      -webkit-appearance: none;
      width: 12px;
      height: 12px;
      cursor: ew-resize;
      background-color: $text-light;
      border-radius: 50%;
    }
  }

  .field {
    position: relative;
    flex-grow: 1;
  }

  .slider-label {
    position: absolute;
    top: -55px;
    transform: translateX(-50%);
    z-index: 1;

    &::after {
      position: absolute;
      content: "";
      display: block;
      height: rem(45);
      width: rem(45);
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%) rotate(45deg);
      padding: 0.5rem;
      z-index: -1;
      background-color: $background;
      border-top-left-radius: 50%;
      border-top-right-radius: 50%;
      border-bottom-left-radius: 50%;
    }
  }
</style>
