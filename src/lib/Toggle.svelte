<script>
  export let on = false;
  let state = on;
</script>

<div class="container" on:click={() => state = !state}>
  <div>Wi-Fi</div>
  {#if state}
    <div class="grid grid-cols-2 w-[56px] h-8 bg-blue-400 rounded-full p-1">
      <div class="h-[24px] w-[24px] rounded-full col-end-1 m-auto"></div>
      <div class="h-[24px] w-[24px] bg-white rounded-full col-end-2 m-auto"></div>
    </div>
  {:else}
    <div class="grid grid-cols-2 w-[56px] h-8 bg-gray-300 rounded-full p-1">
      <div class="h-[24px] w-[24px] bg-white rounded-full col-end-1 m-auto"></div>
      <div class="h-[24px] w-[24px] rounded-full col-end-2 m-auto"></div>
    </div>
  {/if}
</div>

<style>
  .container {
    @apply px-6 flex justify-between pt-10 items-center;
  }
</style>
