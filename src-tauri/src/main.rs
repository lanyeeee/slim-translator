// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Context, Wry};

use crate::config::Config;

mod config;

#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tauri::command(async)]
#[specta::specta]
#[allow(clippy::needless_pass_by_value)]
fn get_config(app: AppHandle) -> Config {
    Config::load(&app).unwrap()
}

#[tauri::command(async)]
#[specta::specta]
#[allow(clippy::needless_pass_by_value)]
fn save_config(app: AppHandle, config: Config) {
    config.save(&app).unwrap();
}

fn generate_context() -> Context<Wry> {
    tauri::generate_context!()
}

fn main() {
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
            Ok(())
        })
        .run(generate_context())
        .expect("error while running tauri application");
}
