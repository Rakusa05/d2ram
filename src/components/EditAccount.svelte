<script lang="ts">
  import BaseModal from "./BaseModal.svelte";
  import type { Account } from "../lib/types";

  let {
    open = $bindable(false),
    selectedAccount,
    onDelete,
    onEdit,
  } = $props<{
    open: boolean;
    selectedAccount: Account | null;
    onDelete: (a: Account) => void | Promise<void>;
    onEdit: (a: Account) => void | Promise<void>;
  }>();

  let a = $state<Account>({
    id: selectedAccount.id,
    displayName: "",
    accountLogin: "",
    password: "",
    running: selectedAccount.running,
    region: selectedAccount.region,
    pid: selectedAccount.pid,
  });

  function copyFields() {
    if (selectedAccount) {
      a.displayName = selectedAccount.displayName;
      a.accountLogin = selectedAccount.accountLogin;
      a.password = selectedAccount.password;
    }
  }

  $effect(() => {
    if (open) copyFields();
  });

  function handleAccept() {
    if (onEdit) onEdit(a);
    open = false;
  }

  async function handleDelete() {
    if (onDelete) await onDelete(a);
    open = false;
  }
</script>

<BaseModal bind:open onAccept={handleAccept} btnActionName="Update">
  <h1 class="text-white font-bold text-3xl">
    Edit {selectedAccount.displayName}
  </h1>
  <div class="flex flex-col justify-center items-center pt-10">
    <span class="flex flex-col space-y-1 pb-3">
      <label for="displayNameEdit" class="text-white">Display Name</label>
      <input
        id="displayNameEdit"
        class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black"
        bind:value={a.displayName}
      />
    </span>
    <span class="flex flex-col space-y-1 pb-3">
      <label for="accountLoginEdit" class="text-white">Account Login</label>
      <input
        id="accountLoginEdit"
        class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black"
        bind:value={a.accountLogin}
      />
    </span>
    <span class="flex flex-col space-y-1 pb-8">
      <label for="passwordEdit" class="text-white">Password</label>
      <input
        id="passwordEdit"
        type="password"
        class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black"
        bind:value={a.password}
      />
    </span>
    <button
      class="bg-red-600 border border-black hover:bg-red-500 rounded-xl px-5 py-1 font-bold w-64"
      onclick={handleDelete}>Delete</button
    >
  </div>
</BaseModal>
