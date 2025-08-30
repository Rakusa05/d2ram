<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import Header from "../components/Header.svelte";
  import AccountCards from "../components/AccountCards.svelte";
  import Footer from "../components/Footer.svelte";
  import AddAccount from "../components/AddAccount.svelte";
  import EditAccount from "../components/EditAccount.svelte";
  import type { Account, Region } from "../lib/types";

  let accounts = $state<Array<Account>>([]);

  let addAccountModal = $state(false);
  let editAccountModal = $state(false);
  let selectedAccount = $state<Account | null>(null);

  function openEdit(account: Account) {
    selectedAccount = account;
    editAccountModal = true;
  }

  function isSameInformation(a: Account | null, b: Account | null) {
    return (
      a?.displayName === b?.displayName &&
      a?.accountLogin === b?.accountLogin &&
      a?.password === b?.password
    );
  }

  async function addAccount(account: Account) {
    accounts = await invoke("add_account", {
      newAccount: {
        displayName: account.displayName,
        accountLogin: account.accountLogin,
        password: account.password,
      },
    });
  }

  async function editAccount(account: Account) {
    if (!isSameInformation(account, selectedAccount)) {
      accounts = await invoke("edit_account", {
        id: account.id,
        updatedAccount: {
          displayName: account.displayName,
          accountLogin: account.accountLogin,
          password: account.password,
        },
      });
    }
  }

  async function deleteAccount(account: Account) {
    accounts = await invoke("delete_account", {
      id: account.id,
    });
  }

  async function launchAccount(id: Account["id"] | String, region: Region) {
    accounts = await invoke("launch_account", {
      id: id.toString(),
      region,
    });
  }

  async function refreshAccount() {
    accounts = await invoke("get_accounts_info");
  }

  onMount(() => {
    refreshAccount();
    const timer = setInterval(() => {
      refreshAccount();
    }, 3000);

    return () => clearInterval(timer);
  });

  $effect(() => {
    if (!editAccountModal) selectedAccount = null;
  });
</script>

<AddAccount bind:open={addAccountModal} onAccept={addAccount} />

{#if selectedAccount}
  <EditAccount
    bind:open={editAccountModal}
    {selectedAccount}
    onDelete={deleteAccount}
    onEdit={editAccount}
  />
{/if}

<main class="text-white h-dvh flex flex-col">
  <div class="h-2 bg-neutral-950 sticky"></div>
  <div class="grow bg-neutral-800 overflow-hidden">
    <div class="h-full overflow-y-auto p-5 custom-scrollbar mx-1">
      <div class="grid [grid-template-columns:repeat(auto-fit,400px)] gap-5">
        {#each accounts as account}
          <AccountCards
            {account}
            onDisplayClick={openEdit}
            onPlayClick={launchAccount}
          />
        {/each}
      </div>
    </div>
  </div>
  <div class="min-h-30 bg-neutral-950 content-center">
    <Footer bind:open={addAccountModal} onPlayClick={launchAccount} />
  </div>
</main>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 8px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: #1c1917;
    border-radius: 6px;
    margin-top: 10px;
    margin-bottom: 10px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background-color: #b91c1c;
    border-radius: 6px;
  }
</style>
