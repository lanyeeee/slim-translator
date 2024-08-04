// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Context, Wry};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

fn generate_context() -> Context<Wry> {
    tauri::generate_context!()
}

fn main() {
    let (invoke_handler, register_events) = {
        let builder = tauri_specta::ts::builder::<Wry>()
            .commands(tauri_specta::collect_commands![greet])
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
