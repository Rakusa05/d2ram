<script lang="ts">
  import type { Region } from "$lib/types";
  import { Button, ButtonGroup } from "flowbite-svelte";
  import { UserAddSolid, CogSolid } from "flowbite-svelte-icons";

  let { open = $bindable(false), onPlayClick } = $props<{
    open: boolean;
    onPlayClick: (a: String, r: Region) => void | Promise<void>;
  }>();

  const launchAccount = (region: Region) => {
    console.log(`Launching game for all accounts\n${region}`);
    onPlayClick("all", region);
  };
</script>

<!-- Need to finish placement for small screen size -->

<div class="grid grid-cols-3 w-full items-center gap-4">
  <div class="flex justify-center">
    <button class=" hover:cursor-pointer" onclick={() => (open = true)}
      ><UserAddSolid
        class="size-10 fill-red-700 stroke-black hover:fill-red-500"
      /></button
    >
  </div>
  <div class="flex flex-col items-center space-y-3">
    <p class="text-center w-full">Open all clients</p>
    <ButtonGroup>
      <Button
        color="red"
        class="w-22"
        pill
        outline
        onclick={() => launchAccount("america")}>America</Button
      >
      <Button
        color="red"
        class="w-22"
        outline
        onclick={() => launchAccount("europe")}>Europe</Button
      >
      <Button
        color="red"
        class="w-22"
        pill
        outline
        onclick={() => launchAccount("asia")}>Asia</Button
      >
    </ButtonGroup>
  </div>
  <div class="flex justify-center">
    <button class="hover:cursor-pointer"
      ><CogSolid
        class="size-10 fill-red-700 stroke-black hover:fill-red-500"
      /></button
    >
  </div>
</div>
