use crate::translator::Translator;
use get_selected_text::get_selected_text;
use mouse_position::mouse_position::Mouse;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tauri::{
    async_runtime::Mutex, plugin::TauriPlugin, AppHandle, Manager, PhysicalPosition, WebviewWindow,
    Wry,
};
use tauri_plugin_global_shortcut::{Code, Shortcut, ShortcutState};

pub fn plugin() -> TauriPlugin<Wry> {
    // 因为with_handler的闭包要求是Fn，而不是FnMut，所以需要使用Mutex包裹变量
    // Because the closure of with_handler requires Fn, not FnMut, so you need to use Mutex to wrap the variable
    let caps_lock_last_press_time = std::sync::Mutex::new(Instant::now());
    let caps_lock_pressed_once = std::sync::Mutex::new(false);
    let double_press_threshold = Duration::from_millis(500);

    tauri_plugin_global_shortcut::Builder::new()
        .with_shortcuts([
            Shortcut::new(None, Code::CapsLock),
            Shortcut::new(None, Code::Escape),
        ])
        .expect("failed to create shortcut")
        .with_handler(move |app_handle, shortcut, event| match shortcut.key {
            Code::CapsLock if event.state == ShortcutState::Released => {
                let mut caps_lock_last_press_time = caps_lock_last_press_time.lock().unwrap();
                let mut caps_lock_pressed_once = caps_lock_pressed_once.lock().unwrap();
                let now = Instant::now();
                let duration = now.duration_since(*caps_lock_last_press_time);

                if *caps_lock_pressed_once && duration < double_press_threshold {
                    double_pressed_caps_lock_callback(app_handle).unwrap();
                    *caps_lock_pressed_once = false;
                } else {
                    *caps_lock_last_press_time = now;
                    *caps_lock_pressed_once = true;
                }
            }

            Code::Escape if event.state == ShortcutState::Released => {
                let panel = app_handle
                    .get_webview_window("panel")
                    .expect("failed to get panel window");
                panel.hide().unwrap();
            }
            _ => {}
        })
        .build()
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

        let result = translator.translate(from, to, selected_text).await?;
        println!("{}", result);
        panel.emit("translate", result)?;

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

    Ok(())
}
