<script>
  export let selection = 0;
  let items = [
    {title: "Dashboard", icon: "⧉"},
    {title: "Setting", icon: "⧉"},
  ];
</script>

<div class="container">
  <div 
    class="h-20 border-b-2 border-gray-400 flex justify-center items-center text-white
    font-bold text-xl mb-10"
  >Ecosync</div>
  <div></div>
  {#each items as item, i}
    {#if i === selection}
      <div class="h-16 bg-red-400 grid grid-cols-2 pl-10 pr-24 justify-center items-center text-white font-bold">
        <span>{item.icon}</span>
        <span>{item.title}</span>
      </div>
    {:else}  
      <div class="h-16 hover:bg-red-400 grid grid-cols-2 pl-10 pr-24 justify-center items-center text-white font-bold">
        <span>{item.icon}</span>
        <span>{item.title}</span>
      </div>
    {/if}
  {/each}
</div>

<style>
  .container {
    @apply h-screen w-64 bg-gray-900 fixed left-0 top-0;
  }
</style>
