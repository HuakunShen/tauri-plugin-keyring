<script>
  import { getSecret, setSecret, deleteSecret } from "tauri-plugin-keyring-api";
  import { toast } from "svelte-sonner";

  let service = $state("tauri-plugin-keyring");
  let user = $state("secret-test");
  let secret = $state("");
  let secretRead = $state("");
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
    <label class="font-bold" for="secret">Secret</label>
    <input class="input input-bordered w-full max-w-xs" bind:value={secret} />
  </div>
  <div>
    <button
      class="btn btn-primary btn-xs"
      onclick={() => {
        getSecret(service, user)
          .then((p) => {
            secretRead = p ? new TextDecoder().decode(p) : "";
          })
          .catch((err) => {
            toast.error("Failed to get secret", {
              description: err.message,
            });
          });
      }}
    >
      Get Secret
    </button>
    <button
      class="btn btn-primary btn-xs"
      onclick={() => {
        if (!secret || secret.length === 0) {
          toast.error("Secret cannot be empty");
          return;
        }
        setSecret(service, user, new TextEncoder().encode(secret))
          .then(() => {
            toast.success("Secret set");
          })
          .catch((err) => {
            toast.error("Failed to set secret", {
              description: err.message,
            });
          });
      }}
    >
      Set Secret
    </button>
    <button
      class="btn btn-primary btn-xs"
      onclick={() => {
        deleteSecret(service, user)
          .then(() => {
            toast.success("Secret deleted");
            secretRead = "";
          })
          .catch((err) => {
            toast.error("Failed to delete secret", {
              description: err.message,
            });
          });
      }}
    >
      Delete Secret
    </button>
  </div>
  <div>
    <p><strong>Secret:</strong> {secretRead}</p>
  </div>
</div>
