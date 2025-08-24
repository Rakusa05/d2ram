<script lang="ts">
  import { Button, ButtonGroup } from "flowbite-svelte";
  import { UserAddSolid, CogSolid } from "flowbite-svelte-icons";
  let { open = $bindable(false) } = $props<{ open: boolean }>();
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
      <Button color="red" class="w-22" pill outline>America</Button>
      <Button color="red" class="w-22" outline>Europe</Button>
      <Button color="red" class="w-22" pill outline>Asia</Button>
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
