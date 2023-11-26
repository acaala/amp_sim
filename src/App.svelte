<script lang="ts">
  import Devices from "./lib/Devices.svelte";
  import Processors from "./lib/Processors.svelte";
</script>

<main class="w-full flex flex-col items-center justify-center">
  <div
    class="flex gap-12 items-center w-full mb-6
   p-4"
  >
    <h1 class="text-4xl font-medium">Amp Sim</h1>
    <div>
      <Devices />
    </div>
  </div>

  <div class="w-full">
    <Processors />
  </div>
</main>
