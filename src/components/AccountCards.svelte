<script lang="ts">
  import { Button, ButtonGroup } from "flowbite-svelte";
  import type { Account } from "../lib/types";

  let { account, onDisplayClick } = $props<{
    account: Account;
    onDisplayClick: (a: Account) => void | Promise<void>;
  }>();

  function test_button(
    running: boolean,
    btnregion: "america" | "europe" | "asia",
    region?: string | null
  ) {
    alert(
      `ID:${account.id}\nAccount:${account.displayName}\nLogin: ${account.accountLogin}\nPassword: ${account.password}\n\n\nRunning: ${account.running}\nRegion: ${account.region}\n\nChanging Running to ${!account.running}\nChanging Region to ${account.btnregion}\n\n${account.pid ? "PID: " + account.pid : ""}`
    );
  }

  const editAccount = () => {
    console.log("editAccount");
    onDisplayClick(account);
  };
</script>

<div
  class="dark:bg-neutral-900 w-100 h-34 dark:text-white rounded-md flex flex-col focus:bg-amber-600"
>
  <div class="flex justify-between mt-2 pl-6 pr-6">
    <p>#{account.id}</p>
    <button
      class="font-bold text-xl hover:text-orange-500 hover:cursor-pointer"
      onclick={() => editAccount()}>{account.displayName}</button
    >
    <div class="flex content-center justify-center pt-1.5 relative">
      <span
        class={`${account.running ? "bg-green-400" : "bg-red-500"} w-3 h-3 rounded-4xl border border-black absolute`}
      ></span>
      <span
        class={`${account.running ? "bg-green-400" : "bg-red-500"} w-3 h-3 rounded-4xl  blur-sm absolute`}
      ></span>
    </div>
  </div>
  <div class="justify-center flex pt-10">
    <ButtonGroup class="">
      <Button
        color={account.running && account.region === "america"
          ? "green"
          : "red"}
        class="w-22 hover:cursor-pointer"
        pill
        outline={!(account.running && account.region === "america")}
        disabled={account.running && !(account.region === "america")}
        onclick={() => test_button(account.running, "america", account.region)}
        >America</Button
      >
      <Button
        color={account.running && account.region === "europe" ? "green" : "red"}
        class="w-22 hover:cursor-pointer"
        outline={!(account.running && account.region === "europe")}
        disabled={account.running && !(account.region === "europe")}
        onclick={() => test_button(account.running, "europe", account.region)}
        >Europe</Button
      >
      <Button
        color={account.running && account.region === "asia" ? "green" : "red"}
        class="w-22 hover:cursor-pointer"
        pill
        outline={!(account.running && account.region === "asia")}
        disabled={account.running && !(account.region === "asia")}
        onclick={() => test_button(account.running, "asia", account.region)}
        >Asia</Button
      >
    </ButtonGroup>
  </div>
</div>
