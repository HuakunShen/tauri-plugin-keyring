<script>
  import "./app.css";
  import { Toaster, toast } from "svelte-sonner";
  import Password from "./lib/components/password.svelte";
  import Secret from "./lib/components/secret.svelte";

  let service = $state("tauri-plugin-keyring");
  let user = $state("test");
  let password = $state("");
  let passwordRead = $state("");
  let secret = $state("");
  let secretRead = $state("");
</script>

<Toaster richColors />
<main class="container mx-auto space-y-2">
  <div class="grid grid-cols-2 gap-4">
    <Password />
    <Secret />
  </div>
</main>
