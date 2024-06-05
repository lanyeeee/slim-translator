// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use error::CommandResult;
use std::sync::Arc;
use tauri::{async_runtime::Mutex, State};
use translator::Translator;

mod config;
mod error;
mod shortcut;
mod translator;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn translate(
    translator: State<'_, Translator>,
    from: &str,
    to: &str,
    content: &str,
) -> CommandResult<String> {
    if from == "auto" {
        let lan = translator.detect_language(content).await?;
        println!("{:?}", lan);
        let result = translator.translate(&lan, to, content).await?;
        Ok(result)
    } else {
        let result = translator.translate(from, to, content).await?;
        Ok(result)
    }
}

fn setup_hook(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();
    app_handle.plugin(shortcut::plugin())?;

    Ok(())
}

fn main() {
    let translator = Arc::new(Translator::new());
    let config = Arc::new(Mutex::new(config::Config::new()));

    tauri::Builder::default()
        .manage(translator)
        .manage(config)
        .setup(setup_hook)
        .invoke_handler(tauri::generate_handler![greet, translate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
