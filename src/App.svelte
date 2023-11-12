<script>
    import Section from "./lib/Section.svelte";
    import Toggle from "./lib/Toggle.svelte";


</script>

<Toggle on />
<Section 
  networks={["Ooredoo_4G_60DA2B_2"]} 
  tag={"Tap to share password"} 
  connected
/>
<Section 
  networks={["Ooredoo_4G_60DA2B", "Honor 6X"]} 
  tag={"Tap to share password"} 
  title={"available networks"}
  add
/>
<Section 
  tag={"Tap to share password"} 
  title={"wifi settings"}
  add
/>

<style lang="postcss">

</style>
