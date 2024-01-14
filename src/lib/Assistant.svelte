<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  export let hideAssistant;
  let apiKey = "";
  let prompt = "";

  async function submitPromptToAi() {
    if (!prompt) return;
    await invoke("submit_user_prompt", { prompt });
  }

  async function setApiKey() {
    // Need to confirm valid key.
    await invoke("set_openai_api_key", { key: apiKey.trim() });
  }

  onMount(async () => {
    apiKey = await invoke("get_openai_api_key");
    if (apiKey) {
      await invoke("init_assistant");
    }
  });
</script>

<div class="absolute inset-0 flex items-center justify-center backdrop-blur-sm">
  <div class="bg-[#333] p-4 rounded-md">
    {#if !apiKey}
      <div class="flex items-center gap-4 w-full justify-between">
        <p class="text-sm font-medium text-gray-300">API Key:</p>
        <input type="password" bind:value={apiKey} on:change={setApiKey} />
      </div>
    {:else}
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-2xl font-medium">Ai Assistant</h2>
        <button on:click={() => (hideAssistant = true)}>Close</button>
      </div>
      <input
        type="text"
        required
        bind:value={prompt}
        placeholder="E.g. Give me a crunchy tone"
      />
      <button class="button" on:click={() => submitPromptToAi()}>Submit</button>
    {/if}
  </div>
</div>
