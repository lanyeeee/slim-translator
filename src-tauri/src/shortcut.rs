use std::sync::Arc;

use crate::translator::Translator;
use mouse_position::mouse_position::Mouse;
use tauri::{plugin::TauriPlugin, AppHandle, Manager, PhysicalPosition, WebviewWindow, Wry};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

pub fn plugin() -> TauriPlugin<Wry> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_shortcut(Shortcut::new(
            Some(Modifiers::CONTROL | Modifiers::ALT),
            Code::CapsLock,
        ))
        .expect("failed to create shortcut")
        .with_handler(move |app_handle, _shortcut, _event| {
            callback(app_handle).unwrap();
        })
        .build()
}

fn callback(app_handle: &AppHandle) -> anyhow::Result<()> {
    // 获取用户选中的文本，如果没有选中的文本则直接返回
    // Get the text selected by the user, if there is no selected text, return directly
    let select_text = selection::get_text();
    if select_text.is_empty() {
        return Ok(());
    }

    let panel = app_handle
        .get_webview_window("panel")
        .expect("failed to get panel window");

    show_panel(&panel)?;

    let translator = Arc::clone(&app_handle.state::<Arc<Translator>>());
    tauri::async_runtime::spawn(async move {
        let result = translator
            .translate("en", "zh", select_text.as_str())
            .await?;
        panel.emit::<String>("translate", result.into())?;

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

    Ok(())
}
