<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let availableProcessors: Array<string> = [];
  let activeProcessors: Array<any> = [];

  async function getAvailableProcessors() {
    availableProcessors = (await invoke("get_processors")) as [];
  }

  onMount(async () => {
    getAvailableProcessors();
  });

  async function addProcessor(e: Event) {
    const target = e.target as HTMLButtonElement;

    if (target) {
      let processorDetails = await invoke("add_processor_to_pipeline", {
        name: target.innerText.toLowerCase(),
      });
      console.log(processorDetails);
      activeProcessors.push({
        name: target.innerText.toLowerCase(),
        processorDetails,
      });
      activeProcessors = activeProcessors;
    }

    console.log(activeProcessors);
  }

  async function updateProcessorValues() {
    let newValues = await invoke("update_processor_values", {
      processorName: "amplifier",
      values: { volume: 1.0 },
    });
  }
</script>

<div>
  <h2 class="text-2xl font-medium mb-4">Audio Effects</h2>

  <div class="flex flex-col gap-4 mb-4">
    {#each availableProcessors as processor}
      <button on:click={(e) => addProcessor(e)}>{processor}</button>
    {/each}
  </div>

  <div>
    <h4 class="text-sm font-medium text-gray-400">Active Processors</h4>
    <div>
      {#each activeProcessors as processor}
        <div><p class="text-lg capitalize">{processor.name}</p></div>
        {#each Object.entries(processor.processorDetails) as [detail, value]}
          <div class="flex">
            <p>{detail}: <span>{value}</span></p>
          </div>
        {/each}
      {/each}
    </div>
  </div>
</div>
