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

      getActiveProcessors();
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
</script>

<div class="flex w-full">
  <div></div>
  <div class="w-full bg-[#0f0f0f] p-4">
    <h2 class="text-2xl font-medium mb-4 text-center">Audio Effects</h2>

    <div class="flex flex-col gap-4 mb-4 w-1/2 mx-auto">
      {#each availableProcessors as processor}
        <button
          class="border border-[#2f2f2f]"
          on:click={(e) => addProcessor(e)}>{processor}</button
        >
      {/each}
    </div>
  </div>

  <div class="w-full p-4">
    <h4 class="text-sm font-medium text-gray-400 text-center mb-4">
      Active Processors
    </h4>
    {#each activeProcessors as processor}
      <form name={processor.name.Str} class="border border-gray-700 p-2">
        <div><p class="text-xl capitalize mb-2">{processor.name.Str}</p></div>
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
