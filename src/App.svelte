<script lang="ts">
  import Devices from "./lib/Devices.svelte";
  import Processors from "./lib/Processors.svelte";
</script>

<main class="w-full mt-24 flex flex-col items-center justify-center">
  <h1 class="text-3xl font-medium mb-4">Amp Sim</h1>

  <div class="mb-12">
    <Devices />
  </div>

  <div class="flex">
    <Processors />
  </div>
</main>
