// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::async_runtime::Mutex;
use translator::Translator;

mod config;
mod shortcut;
mod translator;
mod tray;

fn setup_hook(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();

    shortcut::init(app_handle)?;
    tray::init(app_handle)?;

    Ok(())
}

rust_i18n::i18n!("locales");
fn main() {
    let sys_locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
    rust_i18n::set_locale(&sys_locale);

    let translator = Arc::new(Translator::new());
    let config = Arc::new(Mutex::new(config::Config::new()));

    tauri::Builder::default()
        .manage(translator)
        .manage(config)
        .setup(setup_hook)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
