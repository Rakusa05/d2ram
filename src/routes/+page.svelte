<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { Alert } from "flowbite-svelte";
  import Header from "../components/Header.svelte";
  import AccountCards from "../components/AccountCards.svelte";
  import Footer from "../components/Footer.svelte";
  import AddAccount from "../components/AddAccount.svelte";
  import EditAccount from "../components/EditAccount.svelte";
  import Settings from "../components/Settings.svelte";
  import type { Account, AppSettings, Region } from "../lib/types";

  let accounts = $state<Array<Account>>([]);
  let currentSettings = $state<AppSettings | null>();

  let addAccountModal = $state(false);
  let editAccountModal = $state(false);
  let settingsModal = $state(false);
  let selectedAccount = $state<Account | null>(null);
  let alertOpen = $state(false);
  let alertMsg = $state<string>("");

  function openEdit(account: Account) {
    selectedAccount = account;
    editAccountModal = true;
  }

  function isSameInformation(a: Account | null, b: Account | null) {
    return (
      a?.displayName === b?.displayName &&
      a?.connectionType === b?.connectionType &&
      a?.accountLogin === b?.accountLogin &&
      a?.password === b?.password &&
      a?.token === b?.token
    );
  }

  async function addAccount(account: Account) {
    accounts = await invoke("add_account", {
      newAccount: {
        displayName: account.displayName,
        connectionType: account.connectionType,
        accountLogin: account.accountLogin,
        password: account.password,
        token: account.token,
      },
    });
  }

  async function editAccount(account: Account) {
    if (!isSameInformation(account, selectedAccount)) {
      accounts = await invoke("edit_account", {
        id: account.id,
        updatedAccount: {
          displayName: account.displayName,
          connectionType: account.connectionType,
          accountLogin: account.accountLogin,
          password: account.password,
          token: account.token,
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

  async function loadSetting() {
    currentSettings = await invoke<AppSettings>("get_settings");
  }

  async function saveSettings(settings: AppSettings) {
    console.log("HELLLLO");
    currentSettings = await invoke("save_settings", {
      newSettings: settings,
    });
  }

  function alert(msg: string) {
    alertOpen = true;
    alertMsg = msg;
  }

  onMount(() => {
    refreshAccount();
    loadSetting();
    const timer = setInterval(() => {
      refreshAccount();
    }, 3000);

    return () => clearInterval(timer);
  });

  $effect(() => {
    if (!editAccountModal) selectedAccount = null;
  });

  $effect(() => {
    if (alertOpen) {
      const timer = setInterval(() => {
        alertOpen = false;
      }, 7000);

      return () => clearInterval(timer);
    }
  });
</script>

<AddAccount bind:open={addAccountModal} onAccept={addAccount} onAlert={alert} />

{#if selectedAccount}
  <EditAccount
    bind:open={editAccountModal}
    {selectedAccount}
    onDelete={deleteAccount}
    onEdit={editAccount}
    onAlert={alert}
  />
{/if}

{#if currentSettings}
  <Settings
    bind:open={settingsModal}
    {currentSettings}
    onAccept={saveSettings}
  />
{/if}

<main class="text-white h-dvh flex flex-col">
  <Alert
    bind:alertStatus={alertOpen}
    rounded={false}
    dismissable
    class="absolute w-full z-40 bg-red-900! text-white! border-black! border"
    >{alertMsg}</Alert
  >
  <!--
    <Header />
  -->
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
    <Footer
      bind:openAcc={addAccountModal}
      bind:openSet={settingsModal}
      {accounts}
      onPlayClick={launchAccount}
    />
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
