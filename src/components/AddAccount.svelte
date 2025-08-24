<script lang="ts">
  import type { Account } from "$lib/types";
  import BaseModal from "./BaseModal.svelte";

  let { open = $bindable(), onAccept } = $props<{
    open: boolean;
    onAccept: (a: Account) => void | Promise<void>;
  }>();

  let a = $state<Account>({
    id: 0,
    displayName: "",
    accountLogin: "",
    password: "",
    region: null,
    running: false,
    pid: null,
  });

  function validateData(a?: Account) {
    return a?.displayName != "" && a?.accountLogin != "" && a?.password != "";
  }

  function handleAccept() {
    if (validateData(a)) {
      onAccept(a);
    } else {
      alert("Invalid Account");
    }
  }

  function resetFields() {
    a = {
      id: 0,
      displayName: "",
      accountLogin: "",
      password: "",
      region: null,
      running: false,
      pid: null,
    };
  }

  $effect(() => {
    if (!open) resetFields();
  });
</script>

<BaseModal bind:open onAccept={handleAccept} btnActionName="Add">
  <h1 class="text-white font-bold text-3xl">Add account</h1>
  <div class="flex flex-col justify-center items-center pt-10">
    <span class="flex flex-col space-y-1 pb-3">
      <label for="displayName" class="text-white">Display Name</label>
      <input
        id="displayName"
        class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black"
        bind:value={a.displayName}
      />
    </span>
    <span class="flex flex-col space-y-1 pb-3">
      <label for="accountLogin" class="text-white">Account Login</label>
      <input
        id="accountLogin"
        class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black"
        bind:value={a.accountLogin}
      />
    </span>
    <span class="flex flex-col space-y-1">
      <label for="password" class="text-white">Password</label>
      <input
        id="password"
        type="password"
        class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black"
        bind:value={a.password}
      />
    </span>
  </div>
</BaseModal>
