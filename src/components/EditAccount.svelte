<script lang="ts">
  import BaseModal from "./BaseModal.svelte";
  import type { Account } from "../lib/types";

  let {
    open = $bindable(false),
    selectedAccount,
    onDelete,
    onEdit,
    onAlert,
  } = $props<{
    open: boolean;
    selectedAccount: Account | null;
    onDelete: (a: Account) => void | Promise<void>;
    onEdit: (a: Account) => void | Promise<void>;
    onAlert: (msg: string) => void;
  }>();

  let a = $state<Account>({
    id: selectedAccount.id,
    displayName: "",
    connectionType: "login",
    accountLogin: "",
    password: "",
    token: "",
    running: selectedAccount.running,
    region: selectedAccount.region,
    pid: selectedAccount.pid,
  });

  let alertMsg = $state("");

  function copyFields() {
    if (selectedAccount) {
      a.displayName = selectedAccount.displayName;
      a.connectionType = selectedAccount.connectionType;
      a.accountLogin = selectedAccount.accountLogin;
      a.password = selectedAccount.password;
      a.token = selectedAccount.token;
    }
  }

  function validateData(a?: Account) {
    if (a?.displayName == "") {
      alertMsg = "Enter a Display Name please";
      return false;
    } else if (
      a?.connectionType == "login" &&
      (a?.accountLogin == "" || a?.password == "")
    ) {
      alertMsg =
        "Please fill Account Login and Password with Login connection type";
      return false;
    } else if (a?.connectionType == "token" && a?.token == "") {
      alertMsg = "Please fill Token with Web Token connection type";
    } else return true;
  }

  function handleAccept() {
    if (validateData(a)) {
      onEdit(a);
      open = false;
    } else {
      onAlert(alertMsg);
    }
  }

  async function handleDelete() {
    if (onDelete) await onDelete(a);
    open = false;
  }

  $effect(() => {
    if (open) copyFields();
  });
</script>

<BaseModal bind:open onAccept={handleAccept} btnActionName="Update">
  <h1 class="text-white font-bold text-3xl">
    Edit {selectedAccount.displayName}
  </h1>
  <div class="flex flex-col justify-center items-center pt-10">
    <div class="space-y-3 pb-8">
      <div class="flex flex-col space-y-1">
        <label for="displayNameEdit" class="text-white">Display Name</label>
        <input
          id="displayNameEdit"
          class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black pl-3"
          bind:value={a.displayName}
        />
      </div>
      <div class="flex flex-col space-y-1">
        <label for="connTypeEdit" class="text-white">Connection Type</label>
        <select
          id="connTypeEdit"
          class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black pl-3"
          bind:value={a.connectionType}
        >
          <option value="login">Account Login</option>
          <option value="token">Web Token</option>
        </select>
      </div>
      <div class="flex flex-col space-y-1">
        <label for="accountLoginEdit" class="text-white">Account Login</label>
        <input
          id="accountLoginEdit"
          class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black pl-3"
          bind:value={a.accountLogin}
        />
      </div>
      <div class="flex flex-col space-y-1">
        <label for="passwordEdit" class="text-white">Password</label>
        <input
          id="passwordEdit"
          type="password"
          class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black"
          bind:value={a.password}
        />
      </div>
      <div class="flex flex-col space-y-1">
        <label for="tokenEdit" class="text-white">Token</label>
        <input
          id="tokenEdit"
          type="password"
          class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black"
          bind:value={a.token}
        />
      </div>
    </div>
    <button
      class="bg-red-600 border border-black hover:bg-red-500 rounded-xl px-5 py-1 font-bold w-64 hover:cursor-pointer"
      onclick={handleDelete}>Delete</button
    >
  </div>
</BaseModal>
