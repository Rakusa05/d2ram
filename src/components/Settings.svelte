<script lang="ts">
  import BaseModal from "./BaseModal.svelte";
  import type { AppSettings } from "$lib/types";

  let {
    open = $bindable(false),
    currentSettings,
    onAccept,
  } = $props<{
    open: boolean;
    currentSettings: AppSettings | null;
    onAccept: (s: AppSettings) => void | Promise<void>;
  }>();

  let s = $state<AppSettings>({
    gamePath: currentSettings.gamePath,
    gameLanguage: currentSettings.gameLanguage,
  });

  async function handleAccept() {
    if (onAccept) onAccept(s);
  }
</script>

<BaseModal bind:open onAccept={handleAccept} btnActionName={"Save"}>
  <h1 class="text-white font-bold text-3xl">Settings</h1>
  <div class="flex flex-col justify-center items-center pt-10 space-y-3">
    <span class="flex flex-col space-y-1">
      <label for="gamePath" class="text-white">Game Path</label>
      <input
        id="gamePath"
        class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black"
        bind:value={s.gamePath}
      />
    </span>
    <div class="flex flex-col space-y-1">
      <label for="lang" class="text-white">Game Language</label>
      <select
        id="lang"
        class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black pl-3"
        bind:value={s.gameLanguage}
      >
        <option value="default">Default</option>
        <option value="english">English</option>
        <option value="french">Français</option>
      </select>
    </div>
  </div>
</BaseModal>
