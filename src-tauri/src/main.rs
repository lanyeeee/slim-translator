// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use error::CommandResult;
use tauri::{plugin::TauriPlugin, LogicalSize, Manager, State, Wry};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};
use translator::Translator;

mod error;
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

fn shortcut_plugin() -> TauriPlugin<Wry> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_shortcut(Shortcut::new(
            Some(Modifiers::CONTROL | Modifiers::ALT),
            Code::CapsLock,
        ))
        .unwrap()
        .with_handler(move |app_handle, _shortcut, _event| {
            let translator = Arc::clone(&app_handle.state::<Arc<Translator>>());
            tauri::async_runtime::spawn(async move {
                let select_text = selection::get_text();
                let result = translator
                    .translate("en", "zh", select_text.as_str())
                    .await
                    .unwrap();
                println!("{:?}", result);
            });
        })
        .build()
}

fn setup_hook(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个线程监听键盘事件
    // Create a thread to listen to keyboard events
    let app_handle = app.handle();
    app_handle.plugin(shortcut_plugin())?;

    let panel = app
        .get_webview_window("panel")
        .expect("failed to get panel window");
    panel.set_size(LogicalSize {
        width: 400,
        height: 200,
    })?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(translator::Translator::new()))
        .setup(setup_hook)
        .invoke_handler(tauri::generate_handler![greet, translate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
