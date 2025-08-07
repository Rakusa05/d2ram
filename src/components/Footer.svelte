<script lang="ts">
  import { Button, ButtonGroup, Label, Select } from "flowbite-svelte";
  import { UserAddSolid, CogSolid } from "flowbite-svelte-icons";
</script>

<div class="flex justify-evenly">
  <div class="justify-center content-center">
    <div class="flex space-x-4 content-center text-center justify-center">
      <p class="justify-center text-center content-center">Open all clients</p>
      <ButtonGroup class="justify-center content-center">
        <Button color="red" class="w-22" pill outline>America</Button>
        <Button color="red" class="w-22" outline>Europe</Button>
        <Button color="red" class="w-22" pill outline>Asia</Button>
      </ButtonGroup>
    </div>
  </div>
  <div class="justify-center content-center text-center space-x-7">
    <button class=" hover:cursor-pointer"
      ><UserAddSolid
        class="size-10 fill-red-500 stroke-red-600 hover:fill-red-700"
      /></button
    >
    <button class="hover:cursor-pointer"
      ><CogSolid
        class="size-10 fill-red-500 stroke-red-600 hover:fill-red-700 "
      /></button
    >
  </div>
</div>
