<script>
  import {
    getPassword,
    setPassword,
    deletePassword,
  } from "tauri-plugin-keyring-api";
  import { toast } from "svelte-sonner";

  let service = $state("tauri-plugin-keyring");
  let user = $state("test");
  let password = $state("");
  let passwordRead = $state("");
</script>

<div class="space-y-2">
  <div class="flex flex-col space-y-1.5">
    <label class="font-bold" for="service">Service</label>
    <input class="input input-bordered w-full max-w-xs" bind:value={service} />
  </div>
  <div class="flex flex-col space-y-1.5">
    <label class="font-bold" for="user">User</label>
    <input class="input input-bordered w-full max-w-xs" bind:value={user} />
  </div>
  <div class="flex flex-col space-y-1.5">
    <label class="font-bold" for="password">Password</label>
    <input class="input input-bordered w-full max-w-xs" bind:value={password} />
  </div>
  <div>
    <button
      class="btn btn-primary btn-xs"
      onclick={() => {
        getPassword(service, user)
          .then((p) => {
            passwordRead = p ?? "";
          })
          .catch((err) => {
            toast.error("Failed to get password", {
              description: err.message,
            });
          });
      }}
    >
      Get Password
    </button>
    <button
      class="btn btn-primary btn-xs"
      onclick={() => {
        if (!password || password.length === 0) {
          toast.error("Password cannot be empty");
          return;
        }
        setPassword(service, user, password)
          .then(() => {
            toast.success("Password set");
          })
          .catch((err) => {
            toast.error("Failed to set password", {
              description: err.message,
            });
          });
      }}
    >
      Set Password
    </button>
    <button
      class="btn btn-primary btn-xs"
      onclick={() => {
        deletePassword(service, user)
          .then(() => {
            toast.success("Password deleted");
            passwordRead = "";
          })
          .catch((err) => {
            toast.error("Failed to delete password", {
              description: err.message,
            });
          });
      }}
    >
      Delete Password
    </button>
  </div>
  <div>
    <p><strong>Password:</strong> {passwordRead}</p>
  </div>
</div>
