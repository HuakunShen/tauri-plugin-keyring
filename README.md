# Tauri Plugin keyring

![NPM Version](https://img.shields.io/npm/v/tauri-plugin-keyring-api)
![Crates.io Version](https://img.shields.io/crates/v/tauri-plugin-keyring)

A simple wrapper over rust [keyring](https://crates.io/crates/keyring) crate. This may be useful for many applications that require storing user's sensitive data on disk, so although it's simple, I made a plugin for it.

Using keyring allows you to store user's password in the system keychain safely without prompting user for password everytime.

Tauri's [stronghold plugin](https://tauri.app/plugin/stronghold/) is also used for storing secrets and keys. But it requires user to enter a password or storing the encryption key somewhere. keyring is a good place to store this encryption key.

**Sample Usage:**

- Storing random database encryption key
- Storing user's password for auto-login
- Storing user's auth token

**Sample Project that uses this plugin:** [kunkunsh/kunkun](https://github.com/kunkunsh/kunkun)

## Installation

- Crate: https://crates.io/crates/tauri-plugin-keyring
  - `cargo add tauri-plugin-keyring`
- NPM Package: https://www.npmjs.com/package/tauri-plugin-keyring-api
  - `npm install tauri-plugin-keyring-api`

## Usage

### TypeScript/JavaScript

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

### Rust

```rust
use tauri::Manager;
use tauri_plugin_keyring::KeyringExt;
// app is a tauri::AppHandle
let pass: Option<String> = app.keyring().get_password("tauri-plugin-keyring", "test")?;
```
