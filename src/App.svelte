<script lang="ts">
  import Devices from "./lib/Devices.svelte";
  import Processors from "./lib/Processors.svelte";
</script>

<main class="w-full h-screen flex flex-col items-center justify-center">
  <div
    class="flex gap-12 items-center w-full 
   p-4 border-b border-gray-900"
  >
    <h1 class="text-2xl font-medium">Rustone</h1>
    <div>
      <Devices />
    </div>
  </div>

  <div class="w-full h-full">
    <Processors />
  </div>
</main>
