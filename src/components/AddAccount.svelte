<script lang="ts">
  import type { Account } from "$lib/types";
  import { Toggle } from "flowbite-svelte";
  import BaseModal from "./BaseModal.svelte";

  let {
    open = $bindable(),
    onAccept,
    onAlert,
  } = $props<{
    open: boolean;
    onAccept: (a: Account) => void | Promise<void>;
    onAlert: (msg: string) => void;
  }>();

  let a = $state<Account>({
    id: 0,
    displayName: "",
    connectionType: "login",
    accountLogin: "",
    password: "",
    token: "",
    region: null,
    running: false,
    pid: null,
  });

  let alertMsg = $state("");

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
      onAccept(a);
      open = false;
    } else {
      onAlert(alertMsg);
    }
  }

  function resetFields() {
    a = {
      id: 0,
      displayName: "",
      connectionType: "login",
      accountLogin: "",
      password: "",
      token: "",
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
  <div class="flex flex-col justify-center items-center pt-10 space-y-3">
    <span class="flex flex-col space-y-1">
      <label for="displayName" class="text-white">Display Name</label>
      <input
        id="displayName"
        class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black"
        bind:value={a.displayName}
      />
    </span>
    <div class="flex flex-col space-y-1">
      <label for="connType" class="text-white">Connection Type</label>
      <select
        id="connType"
        class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black pl-3"
        bind:value={a.connectionType}
      >
        <option value="login">Account Login</option>
        <option value="token">Web Token</option>
      </select>
    </div>
    <span class="flex flex-col space-y-1">
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
    <div class="flex flex-col space-y-1">
      <label for="token" class="text-white">Token</label>
      <input
        id="token"
        type="password"
        class="border-red-600 border rounded-xl h-9 w-64 bg-gray-200 text-black"
        bind:value={a.token}
      />
    </div>
  </div>
</BaseModal>
