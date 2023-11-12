<script>
  import Wifi from "./assets/wifi.png"
  import Lock from "./assets/padlock.png"
  export let networks = [];
  export let title = "connected";
  export let tag;
  export let add = false;
  export let connected = false;
</script>

<div class="container">
  <div class="line"></div>
  <div class="title">{title}</div>
  {#each networks as network}
    <div class="network">
      <div class="flex gap-2">
        <div class="flex items-center px-2"><img src={Wifi} alt="" width="20px"></div>
        <div class="flex flex-col name" style="--color: {['#60a5fa', 'black'][Number(!connected)]}">
          {network}
          {#if tag}
            <div class="text-sm">{tag}</div>
          {/if}
        </div>
      </div>
      <div class="flex gap-2 items-center">
        <div class="flex opacity-60 px-2"><img src={Lock} alt="" width="15px"></div>
        <div class="bg-gray-200 rounded-full w-6 h-6 justify-center items-center flex">&gt;</div>
      </div>
    </div>
  {/each}
  {#if add}
    <div class="flex justify-between px-6 hover:bg-gray-200 py-4">
      <div>Add network</div>
      <div>&gt;</div>
    </div>
  {/if}
</div>

<style>
  .container {
    /* @apply px-6; */
  }
  .line {
    @apply h-[1px] bg-gray-200 mt-8 mx-6;
    widows: calc(100vw - 48px);
  }
  .title {
    @apply text-gray-400 uppercase font-bold pt-6 px-6;
  }
  .network {
    @apply flex flex-row justify-between py-4 px-6;
  }
  .network:hover {
    @apply bg-gray-200;
  }
  .name {
    color: var(--color, black);
  }
</style>
