<script>
  import Wifi from "./assets/wifi.png"
  import Lock from "./assets/padlock.png"
  export let networks = [];
  export let title = "connected";
  export let tag;
  export let add = false;
  export let connected = false;

  let tapped = false;
  let selected;
  let password = "";
</script>

<div class="container">
  <div class="line"></div>
  <div class="title">{title}</div>
  {#each networks as network}
    <div class="network" on:click={() => {
      tapped = true;
      selected = network;
    }}>
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
{#if tapped}
  <div class="w-screen fixed bottom-0 h-screen bg-black opacity-40 blur-sm"></div>
  <div 
    class="w-screen fixed bottom-0 h-[300px] bg-white rounded-t-2xl shadow-xl opacity-100 flex flex-col items-center pt-10 gap-12"
  >
    <div class="font-bold">{selected}</div>
    <input 
      type="password" 
      class="border-blue-400 border-2 rounded-lg h-12 w-[80%] bg-gray-200 px-4"
      bind:value={password}
    >
    <div class="grid grid-cols-2 w-[80%]">
      <div class="m-auto py-4 px-10 rounded-full bg-gray-200">Cancel</div>
      <div class="m-auto py-4 px-10 rounded-full bg-gray-200">
        <span 
          class="connect" 
          style="
            --opacity: {['1', '0.5'][Number(password.length <= 4)]};
            --color: {['#60a5fa', 'black'][Number(password.length <= 4)]};
            --weight: {['700', '400'][Number(password.length <= 4)]};
          "
        >Connect</span>
      </div>
    </div>
  </div>
{/if}

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
  .connect {
    opacity: var(--opacity, 0.5);
    color: var(--color, black);
    font-weight: var(--weight, 400);
  }
</style>
