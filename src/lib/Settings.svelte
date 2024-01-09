<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Devices from "./Devices.svelte";
  import { onMount } from "svelte";

  export let hideSettings: boolean;

  let apiKey = "";
  async function setApiKey() {
    await invoke("set_openai_api_key", { key: apiKey.trim() });
  }

  onMount(async () => {
    apiKey = await invoke("get_openai_api_key");
  });
</script>

<div
  class="absolute inset-0 backdrop-blur-sm shadow-xl flex justify-center items-center"
>
  <div class="relative">
    <button
      class="absolute top-2 right-2"
      on:click={() => (hideSettings = true)}
    >
      close
    </button>
    <div class="bg-[#333] p-4 rounded-md">
      <div class="pb-2 mb-4">
        <h3 class="text-3xl font-medium">Settings</h3>
      </div>

      <div class="mb-4">
        <h3 class="text-xl font-medium mb-1">Devices</h3>
        <div class="w-full h-0.5 bg-zinc-500 mb-4"></div>
        <Devices />
      </div>

      <div class="mb-4">
        <h3 class="text-xl font-medium mb-1">AI</h3>
        <div class="w-full h-0.5 bg-zinc-500 mb-4"></div>

        <div class="flex items-center gap-4 w-full justify-between">
          <p class="text-sm font-medium text-gray-300">API Key:</p>
          <input type="password" bind:value={apiKey} on:change={setApiKey} />
        </div>
      </div>
    </div>
  </div>
</div>
