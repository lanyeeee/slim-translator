// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use error::CommandResult;
use tauri::State;
use translator::Translator;

mod error;
mod translator;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
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

fn main() {
    tauri::Builder::default()
        .manage(translator::Translator::new())
        .invoke_handler(tauri::generate_handler![greet, translate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
