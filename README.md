# Tauri Plugin keyring

A simple wrapper over rust [keyring](https://crates.io/crates/keyring) crate.

```ts
import {
  getPassword,
  setPassword,
  deletePassword,
} from "tauri-plugin-keyring-api";

const service = "my-service";
const user = "my-user";

if (!pass) {
  await setPassword(service, user, "my-password");
}
const pass: string = await getPassword(service, user);

await deletePassword(service, user);
```
