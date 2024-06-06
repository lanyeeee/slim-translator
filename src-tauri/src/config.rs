use std::sync::Arc;
use tauri::{async_runtime::Mutex, AppHandle, Manager};

pub struct Config {
    pub from: String,
    pub to: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            from: "auto".to_string(),
            to: "zh".to_string(),
        }
    }

    pub fn set_from(app_handle: &AppHandle, from: &str) {
        let config = app_handle.state::<Arc<Mutex<Config>>>();
        let mut config = config.blocking_lock();
        config.from = from.to_string();
    }

    pub fn set_to(app_handle: &AppHandle, to: &str) {
        let config = app_handle.state::<Arc<Mutex<Config>>>();
        let mut config = config.blocking_lock();
        config.to = to.to_string();
    }
}
