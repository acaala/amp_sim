<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { onMount } from "svelte";

  let name = "";
  let devices: {
    inputs: Array<string>
    outputs: Array<string>
  } = {
    inputs: [],
    outputs: [],
  };


  async function getDevices(){
    devices = await invoke("get_audio_devices");
  }

  onMount( async () => {
    await getDevices();
  })

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
</script>

<div>
  <select on:change={(e) => setInputDevice(e)}>
    {#each devices.inputs as input}
      <option value="{input}">{input}</option>
    {/each}
  </select>
  <select on:change={(e) => setOutDevice(e)}>
    {#each devices.outputs as output}
      <option value="{output}">{output}</option>
    {/each}
  </select>
</div>