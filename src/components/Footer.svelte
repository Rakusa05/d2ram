<script lang="ts">
  import type { Account, Region } from "$lib/types";
  import { Button, ButtonGroup } from "flowbite-svelte";
  import { UserAddSolid, CogSolid } from "flowbite-svelte-icons";

  let {
    open = $bindable(false),
    accounts,
    onPlayClick,
  } = $props<{
    open: boolean;
    accounts: Array<Account>;
    onPlayClick: (a: String, r: Region) => void | Promise<void>;
  }>();

  const launchAccount = (region: Region) => {
    console.log(`Launching game for all accounts\n${region}`);
    onPlayClick("all", region);
  };

  let status = $derived<{
    allRunning: boolean;
    allAmerica: boolean;
    allEurope: boolean;
    allAsia: boolean;
    americaCount: number;
    europeCount: number;
    asiaCount: number;
    accountNumber: number;
  }>({
    allRunning: accounts.every((acc: Account) => acc.running),
    allAmerica: accounts.every((acc: Account) => acc.region === "america"),
    allEurope: accounts.every((acc: Account) => acc.region === "europe"),
    allAsia: accounts.every((acc: Account) => acc.region === "asia"),
    americaCount: accounts.filter((acc: Account) => acc.region === "america")
      .length,
    europeCount: accounts.filter((acc: Account) => acc.region === "europe")
      .length,
    asiaCount: accounts.filter((acc: Account) => acc.region === "asia").length,
    accountNumber: accounts.length,
  });
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
        color={status.americaCount === status.accountNumber && status.allRunning
          ? "green"
          : status.americaCount > 0 &&
              status.americaCount < status.accountNumber
            ? "orange"
            : "red"}
        class="w-22 hover:cursor-pointer"
        outline={!(
          status.americaCount === status.accountNumber && status.allRunning
        )}
        disabled={status.allRunning}
        pill
        onclick={() => launchAccount("america")}>America</Button
      >
      <Button
        color={status.europeCount === status.accountNumber && status.allRunning
          ? "green"
          : status.europeCount > 0 && status.europeCount < status.accountNumber
            ? "orange"
            : "red"}
        class="w-22 hover:cursor-pointer"
        outline={!(
          status.europeCount === status.accountNumber && status.allRunning
        )}
        disabled={status.allRunning}
        onclick={() => launchAccount("europe")}>Europe</Button
      >
      <Button
        color={status.asiaCount === status.accountNumber && status.allRunning
          ? "green"
          : status.asiaCount > 0 && status.asiaCount < status.accountNumber
            ? "orange"
            : "red"}
        class="w-22 hover:cursor-pointer"
        outline={!(
          status.asiaCount === status.accountNumber && status.allRunning
        )}
        disabled={status.allRunning}
        pill
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
