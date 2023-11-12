<script>
    import Bar from "./lib/Bar.svelte";
    import Chart from "./lib/Chart.svelte";
import Navbar from "./lib/Navbar.svelte";

  
</script>

<div class="flex">
  <Navbar />
  <div>
    <Bar />
    <div>
      <div class="w-96 h-92">
        <Chart />
      </div>
    </div>
  </div>
</div>

<style lang="postcss">
  
</style>
