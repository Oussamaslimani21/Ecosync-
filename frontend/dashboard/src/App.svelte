<script>
    import Bar from "./lib/Bar.svelte";
    import Chart from "./lib/Chart.svelte";
import Navbar from "./lib/Navbar.svelte";

  
</script>

<div class="flex">
  <Navbar />
  <div>
    <Bar />
    <div class="w-full min-h-screen p-10 ml-64">
      <div class="w-[600px] h-[700px]">
        <Chart />
      </div>
    </div>
  </div>
</div>

<style lang="postcss">
  
</style>
