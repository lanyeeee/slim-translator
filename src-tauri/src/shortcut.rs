use crate::translator::Translator;
use get_selected_text::get_selected_text;
use mouse_position::mouse_position::Mouse;
use rdev::{EventType, Key};
use rust_i18n::t;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tauri::{async_runtime::Mutex, AppHandle, Manager, PhysicalPosition, WebviewWindow};

pub fn init(app_handle: &AppHandle) -> anyhow::Result<()> {
    let app_handle = app_handle.clone();
    std::thread::spawn(move || {
        let mut ctrl_pressed = false; // 添加一个标志来跟踪 Ctrl 键是否被按下
        let mut caps_lock_last_press_time = Instant::now();
        let mut caps_lock_pressed_once = false;
        let double_press_threshold = Duration::from_millis(500);

        rdev::listen(move |event| match event.event_type {
            // 处理按键按下事件
            // Handle key press events
            EventType::KeyPress(key) => match key {
                Key::ControlLeft | Key::ControlRight => {
                    ctrl_pressed = true;
                }
                Key::CapsLock if ctrl_pressed => {
                    let now = Instant::now();
                    let duration = now.duration_since(caps_lock_last_press_time);

                    if caps_lock_pressed_once && duration < double_press_threshold {
                        double_pressed_caps_lock_callback(&app_handle).unwrap();

                        caps_lock_pressed_once = false;
                    } else {
                        caps_lock_last_press_time = now;
                        caps_lock_pressed_once = true;
                    }
                }
                _ => {}
            },
            EventType::KeyRelease(key) => match key {
                Key::ControlLeft | Key::ControlRight => {
                    ctrl_pressed = false;
                }
                _ => {}
            },
            // 忽略其他事件
            // Ignore other events
            _ => {}
        })
        .expect("failed to listen to events");
    });

    Ok(())
}

fn double_pressed_caps_lock_callback(app_handle: &AppHandle) -> anyhow::Result<()> {
    // 获取用户选中的文本，如果没有选中的文本则直接返回
    // Get the text selected by the user, if there is no selected text, return directly
    let selected_text = get_selected_text().unwrap();
    if selected_text.is_empty() {
        println!("No text selected");
        return Ok(());
    }

    let panel = app_handle
        .get_webview_window("panel")
        .expect("failed to get panel window");

    show_panel(&panel)?;

    let translator = Arc::clone(&app_handle.state::<Arc<Translator>>());
    let config = Arc::clone(&app_handle.state::<Arc<Mutex<crate::config::Config>>>());
    tauri::async_runtime::spawn(async move {
        let config = config.lock().await;
        let from = &config.from;
        let to = &config.to;
        let selected_text = selected_text.as_str();

        match translator.translate(from, to, selected_text).await {
            Ok(deepl_result) => {
                let translated_text = &deepl_result.texts[0];
                let mut text = translated_text.text.clone();
                if !translated_text.alternatives.is_empty() {
                    let alternative_i18n = t!("translate.alternative");
                    text += format!("\n\n====={alternative_i18n}=====\n").as_str();
                    for alternative in translated_text.alternatives.iter() {
                        text += format!("{}\n", alternative.text).as_str();
                    }
                }
                panel.emit("translate", text)?;
            }
            Err(e) => {
                panel.emit("translate", format!("translation failed: {}", e))?;
            }
        }

        Ok::<(), anyhow::Error>(())
    });

    Ok(())
}

fn show_panel(panel: &WebviewWindow) -> anyhow::Result<()> {
    // 获取鼠标位置，计算出 panel 的位置
    // Get the mouse position and calculate the position of the panel
    let (x, y) = match Mouse::get_mouse_position() {
        Mouse::Position { mut x, mut y } => {
            x -= 60;
            y += 20;
            (x, y)
        }
        Mouse::Error => {
            println!("Error getting mouse position");
            (0, 0)
        }
    };

    panel.set_position(PhysicalPosition { x, y })?;
    panel.show()?;
    panel.set_focus()?;

    panel.emit("translate", "翻译中...")?;

    Ok(())
}
