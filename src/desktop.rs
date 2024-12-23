use crate::models::*;
use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Keyring<R>> {
    Ok(Keyring(app.clone()))
}

/// Access to the keyring APIs.
pub struct Keyring<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Keyring<R> {
    pub fn get_password(&self, service: &str, user: &str) -> keyring::Result<Option<String>> {
        Ok(keyring::Entry::new(service, user)?.get_password().ok())
    }

    pub fn set_password(&self, service: &str, user: &str, password: &str) -> keyring::Result<()> {
        keyring::Entry::new(service, user)?.set_password(password)?;
        Ok(())
    }

    pub fn delete_password(&self, service: &str, user: &str) -> keyring::Result<()> {
        keyring::Entry::new(service, user)?.delete_credential()?;
        Ok(())
    }

    pub fn get_secret(&self, service: &str, user: &str) -> keyring::Result<Option<Vec<u8>>> {
        Ok(keyring::Entry::new(service, user)?.get_secret().ok())
    }

    pub fn set_secret(&self, service: &str, user: &str, secret: &[u8]) -> keyring::Result<()> {
        Ok(keyring::Entry::new(service, user)?.set_secret(secret)?)
    }

    pub fn delete_secret(&self, service: &str, user: &str) -> keyring::Result<()> {
        Ok(keyring::Entry::new(service, user)?.delete_credential()?)
    }
}
