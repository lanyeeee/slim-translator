// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    sync::Arc,
    thread::spawn,
    time::{Duration, Instant},
};

use error::CommandResult;
use rdev::{EventType, Key};
use tauri::{async_runtime, LogicalSize, Manager, State};
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

fn register_hotkey(app_handle: &tauri::AppHandle) {
    let translator = Arc::clone(&app_handle.state::<Arc<Translator>>());
    spawn(move || {
        let mut last_press_time = Instant::now();
        let mut pressed_once = false;
        let double_press_threshold = Duration::from_millis(500);
        rdev::listen(move |event| match event.event_type {
            // 处理按键按下事件
            // Handle key press events
            EventType::KeyPress(key) => {
                // TODO: 改成可配置的快捷键
                if key != Key::CapsLock {
                    return;
                }

                let now = Instant::now();
                let duration = now.duration_since(last_press_time);

                if pressed_once && duration < double_press_threshold {
                    let translator = Arc::clone(&translator);
                    tauri::async_runtime::spawn(async move {
                        let select_text = selection::get_text();
                        let result = translator
                            .translate("en", "zh", select_text.as_str())
                            .await
                            .unwrap();
                        println!("{:?}", result);
                    });

                    pressed_once = false;
                } else {
                    last_press_time = now;
                    pressed_once = true;
                }
            }
            // 忽略其他事件
            // Ignore other events
            _ => {}
        })
        .expect("failed to listen to events");
    });
}

fn setup_hook(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个线程监听键盘事件
    // Create a thread to listen to keyboard events
    let app_handle = app.handle();

    register_hotkey(app_handle);

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
