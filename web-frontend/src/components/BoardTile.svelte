<script lang="ts">
  import { tweened, spring } from "svelte/motion";
  import { cubicOut } from "svelte/easing";
  import { onMount } from "svelte";

  export let value: number;
  export let index: number;

  const indexes: Record<number, { top: number; left: number }> = {
    0: { top: 0, left: 0 },
    1: { top: 0, left: 1 },
    2: { top: 0, left: 2 },
    3: { top: 0, left: 3 },
    4: { top: 1, left: 0 },
    5: { top: 1, left: 1 },
    6: { top: 1, left: 2 },
    7: { top: 1, left: 3 },
    8: { top: 2, left: 0 },
    9: { top: 2, left: 1 },
    10: { top: 2, left: 2 },
    11: { top: 2, left: 3 },
    12: { top: 3, left: 0 },
    13: { top: 3, left: 1 },
    14: { top: 3, left: 2 },
    15: { top: 3, left: 3 }
  };

  // const topTweened = tweened(
  //   (tile.new ? tile.position.top : tile.prevPosition.top) / 3,
  //   {
  //     duration: 200,
  //     easing: cubicOut,
  //   }
  // );

  // const leftTweened = tweened(
  //   (tile.new ? tile.position.left : tile.prevPosition.left) / 3,
  //   {
  //     duration: 200,
  //     easing: cubicOut,
  //   }
  // );

  // const mergeSpring = spring(
  //   { scale: 0 },
  //   {
  //     stiffness: 0.3,
  //     damping: 0.3,
  //   }
  // );

  // let mergedValue: number = 0;

  // // Based on composite rule of three
  $: ({ top, left } = indexes[index]);
  $: topValue = top / 3 * 405 + 15;
  $: leftValue = left / 3 * 405 + 15;
  // $: wasMerged = tile.merged && mergedValue !== tile.value;

  // $: if (wasMerged) {
  //   mergeSpring.set({ scale: 1 });
  // }

  // $: if ($mergeSpring.scale === 1) {
  //   mergedValue = tile.value;
  //   mergeSpring.set({ scale: 0 });
  // }

  // onMount(() => {
  //   topTweened.set(tile.position?.top / 3);
  //   leftTweened.set(tile.position?.left / 3);
  // });

  // const scale = (node, { duration }) => {
  //   return {
  //     duration,
  //     css: (t) => {
  //       const eased = tile.new ? cubicOut(t) : 1;

  //       return `
  //         transform: scale(${eased});
  //         `;
  //     },
  //   };
  // };
</script>


<div
  class="tile tile-{value}"
  style="top: {topValue}px; left: {leftValue}px;">
  {value !== 0 ? 2 ** value : ''}
</div>

<style>
  .tile {
    text-align: center;
    line-height: 120px;
    position: absolute;
    width: 120px;
    height: 120px;
  }
  .tile-1 {
    background: #eee4da;
    box-shadow: 0 0 30px 10px rgba(243, 215, 116, 0),
      inset 0 0 0 1px rgba(255, 255, 255, 0);
    font-size: 80px;
  }
  .tile-2 {
    background: #ede0c8;
    box-shadow: 0 0 30px 10px rgba(243, 215, 116, 0),
      inset 0 0 0 1px rgba(255, 255, 255, 0);
    font-size: 80px;
  }
  .tile-3 {
    color: #f9f6f2;
    background: #f2b179;
    font-size: 80px;
  }
  .tile-4 {
    color: #f9f6f2;
    background: #f59563;
    font-size: 80px;
  }
  .tile-5 {
    color: #f9f6f2;
    background: #f67c5f;
    font-size: 80px;
  }
  .tile-6 {
    color: #f9f6f2;
    background: #f65e3b;
    font-size: 80px;
  }
  .tile-7 {
    color: #f9f6f2;
    background: #edcf72;
    box-shadow: 0 0 30px 10px rgba(243, 215, 116, 0.2381),
      inset 0 0 0 1px rgba(255, 255, 255, 0.14286);
    font-size: 45px;
  }
  .tile-8 {
    color: #f9f6f2;
    background: #edcc61;
    box-shadow: 0 0 30px 10px rgba(243, 215, 116, 0.31746),
      inset 0 0 0 1px rgba(255, 255, 255, 0.19048);
    font-size: 45px;
  }
  .tile-9 {
    color: #f9f6f2;
    background: #edc850;
    box-shadow: 0 0 30px 10px rgba(243, 215, 116, 0.39683),
      inset 0 0 0 1px rgba(255, 255, 255, 0.2381);
    font-size: 45px;
  }
  .tile-10 {
    color: #f9f6f2;
    background: #edc53f;
    box-shadow: 0 0 30px 10px rgba(243, 215, 116, 0.47619),
      inset 0 0 0 1px rgba(255, 255, 255, 0.28571);
    font-size: 35px;
  }
  .tile-11 {
    color: #f9f6f2;
    background: #f46573;
    box-shadow: 0 0 30px 10px rgba(243, 215, 116, 0.55556),
      inset 0 0 0 1px rgba(255, 255, 255, 0.33333);
    font-size: 35px;
  }
  .tile-12 {
    color: #f9f6f2;
    background: #f14b61;
    box-shadow: 0 0 30px 10px rgba(243, 215, 116, 0.55556),
      inset 0 0 0 1px rgba(255, 255, 255, 0.33333);
    font-size: 35px;
  }
  .tile-13 {
    color: #f9f6f2;
    background: #e9443d;
    box-shadow: 0 0 30px 10px rgba(243, 215, 116, 0.55556),
      inset 0 0 0 1px rgba(255, 255, 255, 0.33333);
    font-size: 35px;
  }
  .tile-14 {
    color: #f9f6f2;
    background: #72b3db;
    box-shadow: 0 0 30px 10px rgba(243, 215, 116, 0.55556),
      inset 0 0 0 1px rgba(255, 255, 255, 0.33333);
    font-size: 35px;
  }
  .tile-15 {
    color: #f9f6f2;
    background: #5da0e4;
    box-shadow: 0 0 30px 10px rgba(243, 215, 116, 0.55556),
      inset 0 0 0 1px rgba(255, 255, 255, 0.33333);
    font-size: 35px;
  }
  .tile-16 {
    color: #f9f6f2;
    background: #027dc0;
    box-shadow: 0 0 30px 10px rgba(243, 215, 116, 0.55556),
      inset 0 0 0 1px rgba(255, 255, 255, 0.33333);
    font-size: 35px;
  }
</style>