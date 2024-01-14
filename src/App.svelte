<script lang="ts">
  import Assistant from "./lib/Assistant.svelte";
  import Processors from "./lib/Processors.svelte";
  import Settings from "./lib/Settings.svelte";
  import Hamburger from "./lib/icons/Hamburger.svelte";

  let hideAssistant = true;
  let hideSettings = true;
</script>

<main class="w-full h-screen flex flex-col items-center justify-center">
  <div
    class="flex gap-4 items-center w-full
   p-4 border-b border-gray-900"
  >
    <button
      class="w-6 text-white stroke-white fill-white"
      on:click={() => (hideSettings = false)}
    >
      <Hamburger />
    </button>
    <h1 class="text-2xl font-medium">Rustone</h1>
    <button class="button" on:click={() => (hideAssistant = false)}>AI</button>
  </div>

  <div class="w-full h-full">
    <Processors />
  </div>

  <div class:hidden={hideAssistant}>
    <Assistant bind:hideAssistant />
  </div>

  <div class:hidden={hideSettings}>
    <Settings bind:hideSettings />
  </div>
</main>
