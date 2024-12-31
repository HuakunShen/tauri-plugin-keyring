import { invoke } from "@tauri-apps/api/core";

export function getPassword(
  service: string,
  user: string
): Promise<string | null> {
  return invoke<string | null>("plugin:keyring|get_password", {
    service,
    user,
  });
}

export function getOrSetPassword(
  service: string,
  user: string,
  password: string
): Promise<string> {
  return invoke<string>("plugin:keyring|get_or_set_password", {
    service,
    user,
    password,
  });
}

export function setPassword(
  service: string,
  user: string,
  password: string
): Promise<void> {
  return invoke<void>("plugin:keyring|set_password", {
    service,
    user,
    password,
  });
}

export function deletePassword(service: string, user: string): Promise<void> {
  return invoke<void>("plugin:keyring|delete_password", {
    service,
    user,
  });
}

export function getSecret(
  service: string,
  user: string
): Promise<Uint8Array | null> {
  return invoke<number[] | null>("plugin:keyring|get_secret", {
    service,
    user,
  }).then((result) => (result ? new Uint8Array(result) : null));
}

export function getOrSetSecret(
  service: string,
  user: string,
  secret: Uint8Array
): Promise<Uint8Array> {
  return invoke<Uint8Array>("plugin:keyring|get_or_set_secret", {
    service,
    user,
    secret,
  });
}

export function setSecret(
  service: string,
  user: string,
  secret: Uint8Array
): Promise<void> {
  return invoke<void>("plugin:keyring|set_secret", {
    service,
    user,
    secret,
  });
}

export function deleteSecret(service: string, user: string): Promise<void> {
  return invoke<void>("plugin:keyring|delete_secret", {
    service,
    user,
  });
}
