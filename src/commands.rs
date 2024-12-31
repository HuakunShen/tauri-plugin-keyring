use crate::KeyringExt;
use tauri::{command, AppHandle, Runtime};

#[command]
pub(crate) async fn get_password<R: Runtime>(
    app: AppHandle<R>,
    service: String,
    user: String,
) -> Result<Option<String>, String> {
    app.keyring()
        .get_password(&service, &user)
        .map_err(|e| e.to_string())
}

#[command]
pub(crate) async fn get_or_set_password<R: Runtime>(
    app: AppHandle<R>,
    service: String,
    user: String,
    password: String,
) -> Result<String, String> {
    app.keyring().get_or_set_password(&service, &user, &password).map_err(|e| e.to_string())
}

#[command]
pub(crate) async fn set_password<R: Runtime>(
    app: AppHandle<R>,
    service: String,
    user: String,
    password: String,
) -> Result<(), String> {
    app.keyring()
        .set_password(&service, &user, &password)
        .map_err(|e| e.to_string())
}

#[command]
pub(crate) async fn delete_password<R: Runtime>(
    app: AppHandle<R>,
    service: String,
    user: String,
) -> Result<(), String> {
    app.keyring()
        .delete_password(&service, &user)
        .map_err(|e| e.to_string())
}

#[command]
pub(crate) async fn get_secret<R: Runtime>(
    app: AppHandle<R>,
    service: String,
    user: String,
) -> Result<Option<Vec<u8>>, String> {
    app.keyring()
        .get_secret(&service, &user)
        .map_err(|err| err.to_string())
}

#[command]
pub(crate) async fn get_or_set_secret<R: Runtime>(
    app: AppHandle<R>,
    service: String,
    user: String,
    secret: Vec<u8>,
) -> Result<Vec<u8>, String> {
    app.keyring().get_or_set_secret(&service, &user, &secret).map_err(|e| e.to_string())
}

#[command]
pub(crate) async fn set_secret<R: Runtime>(
    app: AppHandle<R>,
    service: String,
    user: String,
    secret: Vec<u8>,
) -> Result<(), String> {
    app.keyring()
        .set_secret(&service, &user, &secret)
        .map_err(|e| e.to_string())
}

#[command]
pub(crate) async fn delete_secret<R: Runtime>(
    app: AppHandle<R>,
    service: String,
    user: String,
) -> Result<(), String> {
    app.keyring()
        .delete_secret(&service, &user)
        .map_err(|e| e.to_string())
}
