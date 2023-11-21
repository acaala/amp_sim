<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let devices: Array<string> = [];

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    devices = await invoke("get_audio_devices");
    // await invoke("start_audio");
    console.log(devices);
  }

  async function setName(e: MouseEvent) {
    const target = e.target as HTMLElement;

    if (target) {
      await invoke("set_audio_input_device", { newDevice: target.innerHTML });
    }
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>

  {#each devices as device}
    <button on:click={event => setName(event)}>{ device }</button>
  {/each}

</div>