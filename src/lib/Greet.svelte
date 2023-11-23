<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let availableProcessors: Array<string> = [];
  // let activeProcessors: Array<Object> = [];

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
    }
  }

  async function updateProcessorValues() {
    let newValues = await invoke("update_processor_values", {
      processorName: "amplifier",
      values: { volume: 1.0 },
    });
    console.log(newValues);
  }
</script>

<div>
  {#each availableProcessors as processor}
    <button on:click={(e) => addProcessor(e)}>{processor}</button>
  {/each}

  <div>
    <button on:click={updateProcessorValues}>Turn volume to 1</button>
  </div>
</div>
