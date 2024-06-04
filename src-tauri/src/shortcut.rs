use std::sync::Arc;

use tauri::{plugin::TauriPlugin, Manager, Wry};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

use crate::translator::Translator;

pub fn plugin() -> TauriPlugin<Wry> {
    tauri_plugin_global_shortcut::Builder::new()
        .with_shortcut(Shortcut::new(
            Some(Modifiers::CONTROL | Modifiers::ALT),
            Code::CapsLock,
        ))
        .expect("failed to create shortcut")
        .with_handler(move |app_handle, _shortcut, _event| {
            let select_text = selection::get_text();
            if select_text.is_empty() {
                return;
            }

            let translator = Arc::clone(&app_handle.state::<Arc<Translator>>());
            tauri::async_runtime::spawn(async move {
                let result = translator
                    .translate("en", "zh", select_text.as_str())
                    .await?;
                println!("{:?}", result);

                Ok::<(), anyhow::Error>(())
            });
        })
        .build()
}