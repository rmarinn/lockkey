<!-- Loader from: https://codepen.io/bojankrsmanovic/pen/dpzeZm -->

<script lang="ts">
  export let loadingMsg: string = "Loading...";
</script>

<div class="relative">
  <div class="wrapper">
    <div class="box-wrap">
      <div class="box one"></div>
      <div class="box two"></div>
      <div class="box three"></div>
      <div class="box four"></div>
      <div class="box five"></div>
      <div class="box six"></div>
    </div>
  </div>
  <div class="msg italic">{loadingMsg}</div>
</div>

<style lang="scss">
  @import "../assets/scss/variables";

  $grad-duration: 1s;
  $move-speed: 3.5s;

  $w-size: 200px;
  $w-margin: -$w-size/2;
  $b-size: 4rem; //This is % based on $w-size. Use any value you want (that makes sense) and any unit that you would like (test it)
  $box-wrapper-width: $b-size;
  $box-wrapper-height: $b-size;
  $box-wrapper-margin: calc((100% - #{$box-wrapper-height}) / 2)
    calc((100% - #{$box-wrapper-width}) / 2);

  //clip-path: inset() doesn't work on firefox and IE, if anyone feels like making it work with SVG clipping, please drop me a link in comments, would like to see it. :)

  .msg {
    position: absolute;
    left: 50%;
    transform: translateX(-50%) translateY(150%);
  }

  *,
  *:before,
  *:after {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  .wrapper {
    position: absolute;
    left: 50%;
    top: 50%;
    margin: $w-margin;
    width: $w-size;
    height: $w-size;
    background-color: transparent;
    border: none;
    user-select: none;
    -webkit-user-select: none;

    .box-wrap {
      width: $box-wrapper-width;
      height: $box-wrapper-height;
      margin: $box-wrapper-margin;
      position: relative;
      transform: rotate(-45deg);

      .box {
        width: 100%;
        height: 100%;
        position: absolute;
        left: 0;
        top: 0;
        background: rgba(135, 0, 0, 0.6);
        background: linear-gradient(to right, $primary, $secondary, $primary);
        background-position: 0% 50%;
        background-size: 1000% 1000%;
        visibility: hidden;

        &.one {
          animation:
            moveGradient $grad-duration infinite,
            oneMove $move-speed infinite;
        }

        &.two {
          animation:
            moveGradient $grad-duration infinite,
            twoMove $move-speed 0.15s infinite;
        }

        &.three {
          animation:
            moveGradient $grad-duration infinite,
            threeMove $move-speed 0.3s infinite;
        }

        &.four {
          animation:
            moveGradient $grad-duration infinite,
            fourMove $move-speed 0.575s infinite;
        }

        &.five {
          animation:
            moveGradient $grad-duration infinite,
            fiveMove $move-speed 0.725s infinite;
        }

        &.six {
          animation:
            moveGradient $grad-duration infinite,
            sixMove $move-speed 0.875s infinite;
        }
      }
    }
  }

  @keyframes moveGradient {
    to {
      background-position: 100% 0%;
    }
  }

  @keyframes oneMove {
    0% {
      visibility: visible;
      clip-path: inset(0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    14.2857% {
      clip-path: inset(0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    28.5714% {
      clip-path: inset(35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    42.8571% {
      clip-path: inset(35% 70% 35% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    57.1428% {
      clip-path: inset(35% 70% 35% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    71.4285% {
      clip-path: inset(0% 70% 70% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    85.7142% {
      clip-path: inset(0% 70% 70% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    100% {
      clip-path: inset(0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }
  }

  @keyframes twoMove {
    0% {
      visibility: visible;
      clip-path: inset(0% 70% 70% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    14.2857% {
      clip-path: inset(0% 70% 70% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    28.5714% {
      clip-path: inset(0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    42.8571% {
      clip-path: inset(0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    57.1428% {
      clip-path: inset(35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    71.4285% {
      clip-path: inset(35% 70% 35% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    85.7142% {
      clip-path: inset(35% 70% 35% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    100% {
      clip-path: inset(0% 70% 70% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }
  }

  @keyframes threeMove {
    0% {
      visibility: visible;
      clip-path: inset(35% 70% 35% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    14.2857% {
      clip-path: inset(35% 70% 35% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    28.5714% {
      clip-path: inset(0% 70% 70% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    42.8571% {
      clip-path: inset(0% 70% 70% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    57.1428% {
      clip-path: inset(0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }
    71.4285% {
      clip-path: inset(0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    85.7142% {
      clip-path: inset(35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    100% {
      clip-path: inset(35% 70% 35% 0 round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }
  }

  @keyframes fourMove {
    0% {
      visibility: visible;
      clip-path: inset(35% 0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    14.2857% {
      clip-path: inset(35% 0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    28.5714% {
      clip-path: inset(35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    42.8571% {
      clip-path: inset(70% 35% 0% 35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    57.1428% {
      clip-path: inset(70% 35% 0% 35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }
    71.4285% {
      clip-path: inset(70% 0 0 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    85.7142% {
      clip-path: inset(70% 0 0 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    100% {
      clip-path: inset(35% 0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }
  }

  @keyframes fiveMove {
    0% {
      visibility: visible;
      clip-path: inset(70% 0 0 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    14.2857% {
      clip-path: inset(70% 0 0 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    28.5714% {
      clip-path: inset(35% 0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    42.8571% {
      clip-path: inset(35% 0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    57.1428% {
      clip-path: inset(35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    71.4285% {
      clip-path: inset(70% 35% 0% 35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    85.7142% {
      clip-path: inset(70% 35% 0% 35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    100% {
      clip-path: inset(70% 0 0 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }
  }

  @keyframes sixMove {
    0% {
      visibility: visible;
      clip-path: inset(70% 35% 0% 35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    14.2857% {
      clip-path: inset(70% 35% 0% 35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    28.5714% {
      clip-path: inset(70% 0 0 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    42.8571% {
      clip-path: inset(70% 0 0 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    57.1428% {
      clip-path: inset(35% 0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    71.4285% {
      clip-path: inset(35% 0% 35% 70% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    85.7142% {
      clip-path: inset(35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }

    100% {
      clip-path: inset(70% 35% 0% 35% round 5%);
      animation-timing-function: cubic-bezier(0.86, 0, 0.07, 1);
    }
  }
</style>
