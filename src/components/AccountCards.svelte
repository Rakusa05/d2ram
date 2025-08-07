<script lang="ts">
  import { Button, ButtonGroup } from "flowbite-svelte";
  let {
    id,
    displayName,
    accountLogin,
    password,
    region,
    status,
  }: {
    id?: number;
    displayName?: string;
    accountLogin?: string;
    password?: string;
    region?: "america" | "europe" | "asia" | null;
    status: boolean;
  } = $props();

  function test_button(
    status: boolean,
    btnregion: "america" | "europe" | "asia",
    region?: string | null
  ) {
    alert(
      `ID:${id}\nAccount:${displayName}\nLogin: ${accountLogin}\nPassword: ${password}\n\n\nStatus: ${status}\nRegion: ${region}\n\nChanging Status to ${!status}\nChanging Region to ${btnregion}`
    );
  }
</script>

<div
  class="dark:bg-neutral-900 w-100 h-34 dark:text-white rounded-md flex flex-col focus:bg-amber-600"
>
  <div class="flex justify-between mt-2 pl-6 pr-6">
    <p>#{id}</p>
    <p class="font-bold text-xl">{displayName}</p>
    <div class="flex content-center justify-center pt-1.5 relative">
      <div
        class={`${status ? "bg-green-400" : "bg-red-500"} w-3 h-3 rounded-4xl border border-black absolute`}
      ></div>
      <div
        class={`${status ? "bg-green-400" : "bg-red-500"} w-3 h-3 rounded-4xl blur-sm absolute`}
      ></div>
    </div>
  </div>
  <div class="justify-center flex pt-10">
    <ButtonGroup class="">
      <Button
        color={status && region === "america" ? "green" : "red"}
        class="w-22 hover:cursor-pointer"
        pill
        outline={!(status && region === "america")}
        disabled={status && !(region === "america")}
        onclick={() => test_button(status, "america", region)}>America</Button
      >
      <Button
        color={status && region === "europe" ? "green" : "red"}
        class="w-22 hover:cursor-pointer"
        outline={!(status && region === "europe")}
        disabled={status && !(region === "europe")}
        onclick={() => test_button(status, "europe", region)}>Europe</Button
      >
      <Button
        color={status && region === "asia" ? "green" : "red"}
        class="w-22 hover:cursor-pointer"
        pill
        outline={!(status && region === "asia")}
        disabled={status && !(region === "asia")}
        onclick={() => test_button(status, "asia", region)}>Asia</Button
      >
    </ButtonGroup>
  </div>
</div>
