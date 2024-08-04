// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Context, Wry};

use crate::commands::{get_config, greet, save_config};

mod commands;
mod config;
mod tray;

fn generate_context() -> Context<Wry> {
    tauri::generate_context!()
}

rust_i18n::i18n!("locales");
fn main() {
    let sys_locale = sys_locale::get_locale().unwrap_or("en-US".to_owned());
    rust_i18n::set_locale(&sys_locale);

    let (invoke_handler, register_events) = {
        let builder = tauri_specta::ts::builder::<Wry>()
            .commands(tauri_specta::collect_commands![
                greet,
                get_config,
                save_config
            ])
            .events(tauri_specta::collect_events![])
            .header("// @ts-nocheck"); // 跳过检查

        #[cfg(debug_assertions)] // 只有在debug模式下才会生成bindings.ts
        let builder = builder.path("../src/bindings.ts");

        builder.build().unwrap()
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(invoke_handler)
        .setup(|app| {
            register_events(app);
            tray::init(app.handle())?;
            Ok(())
        })
        .run(generate_context())
        .expect("error while running tauri application");
}
