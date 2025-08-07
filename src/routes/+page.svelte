<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Footer from "../components/Footer.svelte";
  import Header from "../components/Header.svelte";
  import AccountCards from "../components/AccountCards.svelte";

  type Account = {
    id: number;
    displayName: string;
    accountLogin: string;
    password: string;
    region: "america" | "europe" | "asia" | null;
    status: boolean;
  };

  let accounts: Account[] = [
    {
      id: 1,
      displayName: "Account1",
      accountLogin: "Account1@gmail.com",
      password: "Pa$$w0rD123!",
      region: "america",
      status: false,
    },
    {
      id: 2,
      displayName: "Account2",
      accountLogin: "Account2@gmail.com",
      password: "Pa$$w0rD123!",
      region: "america",
      status: true,
    },
    {
      id: 3,
      displayName: "Account3",
      accountLogin: "Account3@gmail.com",
      password: "Pa$$w0rD123!",
      region: "america",
      status: true,
    },
    {
      id: 4,
      displayName: "Account4",
      accountLogin: "Account4@gmail.com",
      password: "Pa$$w0rD123!",
      region: "asia",
      status: true,
    },
    {
      id: 5,
      displayName: "Account5",
      accountLogin: "Account5@gmail.com",
      password: "Pa$$w0rD123!",
      region: "asia",
      status: true,
    },
    {
      id: 6,
      displayName: "Account6",
      accountLogin: "Account6@gmail.com",
      password: "Pa$$w0rD123!",
      region: null,
      status: false,
    },
    {
      id: 7,
      displayName: "Account7",
      accountLogin: "Account7@gmail.com",
      password: "Pa$$w0rD123!",
      region: "europe",
      status: true,
    },
    {
      id: 8,
      displayName: "Account8",
      accountLogin: "Account8@gmail.com",
      password: "Pa$$w0rD123!",
      region: null,
      status: false,
    },
  ];
</script>

<main class="text-white h-dvh flex flex-col">
  <div class="h-2 bg-neutral-950 sticky"></div>
  <div class="grow bg-neutral-800 overflow-hidden">
    <div class="h-full overflow-y-auto p-5 custom-scrollbar mx-1">
      <div class="grid [grid-template-columns:repeat(auto-fit,400px)] gap-5">
        {#each accounts as account}
          <AccountCards
            id={account.id}
            displayName={account.displayName}
            accountLogin={account.accountLogin}
            password={account.password}
            region={account.region}
            status={account.status}
          />
        {/each}
      </div>
    </div>
  </div>
  <div class="min-h-30 bg-neutral-950 flex sm:flex-col md:flex-row sticky">
    <Footer />
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
