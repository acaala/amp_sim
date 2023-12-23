<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let availableProcessors: Array<string> = [];
  let activeProcessors: Array<any> = [];

  async function getAvailableProcessors() {
    availableProcessors = (await invoke("get_processors")) as [];
  }

  async function getActiveProcessors() {
    activeProcessors = await invoke("get_active_processors");
  }

  onMount(async () => {
    getAvailableProcessors();
    getActiveProcessors();
  });

  async function addProcessor(e: Event) {
    const target = e.target as HTMLButtonElement;

    if (target) {
      await invoke("add_processor_to_pipeline", {
        name: target.innerText.toLowerCase(),
      });

      await getActiveProcessors();
    }
  }

  async function updateProcessorValues(e: Event) {
    let target = e.target as HTMLInputElement;
    if (e.target) {
      let form = target.closest("form") as HTMLFormElement;

      if (form) {
        let values = form.querySelectorAll("input");

        let objectOfNewValues: { [key: string]: number } = {};
        values.forEach((e) => {
          let name = e.name;
          let value = 0;
          if (e.value) {
            value = e.value as unknown as number;
          }

          objectOfNewValues[name] = value;
        });

        await invoke("update_processor_values", {
          processorName: form.name,
          values: objectOfNewValues,
        });
      }
    }
  }

  async function removeProcessor(e: Event) {
    let target = e.target as HTMLElement;

    if (target) {
      let form = target.closest("form");
      if (form) {
        await invoke("remove_processor", { processorName: form.name });

        await getActiveProcessors();
      }
    }
  }
</script>

<div class="flex w-full h-full">
  <div class="w-1/3 py-4 px-6">
    <h2 class="text-xl text-gray-300 font-medium mb-4 ">Audio Effects</h2>

    <div class="flex flex-col gap-4 mb-4">
      {#each availableProcessors as processor}
        <button
          class="border border-[#2f2f2f] capitalize"
          on:click={(e) => addProcessor(e)}>{processor}</button
        >
      {/each}
    </div>
  </div>

  <div class="w-full h-full p-4 bg-[#0f0f0f] flex flex-col gap-1">
    <h4 class="text-sm font-medium text-gray-400 text-center mb-4">
      Active Processors
    </h4>
    {#each activeProcessors as processor}
      <form name={processor.name.Str} class="bg-[#333] border border-gray-800 p-2">
        <div class="flex justify-between mb-4 items-center">
          <div><p class="text-xl capitalize">{processor.name.Str}</p></div>
          <button on:click={(e) => removeProcessor(e)}>Remove</button>
        </div>
        <div class="flex justify-between">
          {#each Object.entries(processor.details.Map) as [detail, value]}
            <div class="flex flex-col items-center px-2">
              <p>{detail}:</p>
              <input
                name={detail}
                type="number"
                on:keyup={(e) => updateProcessorValues(e)}
                on:change={(e) => updateProcessorValues(e)}
                class="text-lg w-full h-full"
                {value}
              />
            </div>
          {/each}
        </div>
      </form>
    {/each}
  </div>
</div>
