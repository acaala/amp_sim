<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { onMount } from "svelte";

  type InitialDevices = {
    input: string,
    output: string
  }

  let devices: {
    inputs: Array<string>
    outputs: Array<string>
  } = {
    inputs: [],
    outputs: [],
  };

  let initialDevices: InitialDevices = {
    input: "",
    output: ""
  }

  let processors: Array<string> = [];


  async function getDevices(){
    devices = await invoke("get_devices");
  }

  async function getProcessors(){
    processors = await invoke("get_processors") as [];
  }

  onMount(async () => {
    await getDevices();
     await startAudio();
     getProcessors();
  })

  async function startAudio() {
    initialDevices = await invoke("start_audio") as InitialDevices;
    console.log(initialDevices);
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


  async function addProcessor(e: Event) {
    const target = e.target as HTMLButtonElement;

    if (target) {
      let processorDetails = await invoke("add_processor_to_pipeline", { name: target.innerText.toLowerCase() })
      console.log(processorDetails);
    }
  }
</script>

<div>
  <select on:change={(e) => setInputDevice(e)}>
    {#each devices.inputs as input}
      <option value="{input}" selected={input == initialDevices.input}>{input}</option>
    {/each}
  </select>
  <select on:change={(e) => setOutDevice(e)}>
    {#each devices.outputs as output}
      <option value="{output}" selected={output == initialDevices.output}>{output}</option>
    {/each}
  </select>
<div>

  {#each processors as processor }
    <button on:click={(e) => addProcessor(e)}>{processor}</button>
  {/each}

  </div>
</div>