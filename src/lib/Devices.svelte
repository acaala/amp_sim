<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let devices: {
    inputs: Array<string>;
    outputs: Array<string>;
  } = {
    inputs: [],
    outputs: [],
  };

  let initialDevices: InitialDevices = {
    input: "",
    output: "",
  };

  async function startAudio() {
    initialDevices = (await invoke("start_audio")) as InitialDevices;
    console.log(initialDevices);
  }

  async function getDevices() {
    devices = await invoke("get_devices");
  }

  async function setInputDevice(e: Event) {
    const target = e.target as HTMLSelectElement;

    if (target) {
      await invoke("set_input_device", { newDevice: target.value });
    }
  }

  async function setOutDevice(e: Event) {
    const target = e.target as HTMLSelectElement;

    if (target) {
      await invoke("set_output_device", { newDevice: target.value });
    }
  }

  onMount(async () => {
    await getDevices();
    await startAudio();
  });
</script>

<div class="flex flex-col gap-4 items-center">
  <div class="flex items-center gap-4 w-full justify-between">
    <p class="text-sm font-medium text-gray-300">Input:</p>
    <select
      class="cursor-pointer bg-[#0f0f0f] appearance-none p-2 w-48"
      on:change={(e) => setInputDevice(e)}
    >
      {#each devices.inputs as input}
        <option value={input} selected={input == initialDevices.input}
          >{input}
        </option>
      {/each}
    </select>
  </div>

  <div class="flex items-center gap-4 w-full justify-between">
    <p class="text-sm font-medium text-gray-300">Output:</p>
    <select
      class="cursor-pointer bg-[#0f0f0f] appearance-none p-2 w-48"
      on:change={(e) => setOutDevice(e)}
    >
      {#each devices.outputs as output}
        <option value={output} selected={output == initialDevices.output}
          >{output}</option
        >
      {/each}
    </select>
  </div>
</div>
