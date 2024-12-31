const COMMANDS: &[&str] = &[
    "get_password",
    "set_password",
    "delete_password",
    "get_secret",
    "set_secret",
    "delete_secret",
    "get_or_set_password",
    "get_or_set_secret",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
