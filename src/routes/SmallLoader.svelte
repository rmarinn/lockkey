<div class="container">
  <div class="loader one"></div>
  <div class="loader two"></div>
  <div class="loader three"></div>
</div>

<style lang="scss">
  @import "../assets/scss/variables";

  $width: 0.4rem;
  $height: 0.8rem;
  $anim-speed: 1s;
  $stagger: 0.1s;

  .container {
    gap: 0.5rem;
    height: 1rem;
    position: relative;
    width: calc($width * 3 + 0.5rem);
  }

  .loader {
    width: $width;
    height: $height;
    background-color: $text-light;
    border-radius: 0.2rem;
    position: absolute;
    top: 50%;
    transform: translateY(-50%);
  }

  .one {
    left: 0;
    animation: movement $anim-speed ease-in-out 0s infinite;
  }

  .two {
    left: 50%;
    animation: movement $anim-speed ease-in-out $stagger infinite;
  }

  .three {
    left: 100%;
    animation: movement $anim-speed ease-in-out calc($stagger * 2) infinite;
  }

  @keyframes movement {
    from {
      height: $height/2;
      background-color: $text-light;
    }

    50% {
      height: $height;
      background-color: $secondary;
    }

    to {
      height: $height/2;
      background-color: $text-light;
    }
  }
</style>
