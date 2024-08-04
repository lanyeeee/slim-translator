use rust_i18n::t;
use tauri::AppHandle;

use crate::config::Config;

#[tauri::command(async)]
#[specta::specta]
pub async fn greet(name: &str) -> Result<String, String> {
    match crate::translate_without_api_key::translate("auto", "en", "今天的天气真不错").await
    {
        Ok(deepl_result) => {
            let translated_text = &deepl_result.texts[0];
            let mut text = translated_text.text.clone();
            if !translated_text.alternatives.is_empty() {
                let alternative_i18n = t!("translate.alternative");
                text += format!("\n\n====={alternative_i18n}=====\n").as_str();
                for alternative in &translated_text.alternatives {
                    text += format!("{}\n", alternative.text).as_str();
                }
            }
            Ok(text)
        }
        Err(e) => Err(format!("translation failed: {e}")),
    }
}

#[tauri::command(async)]
#[specta::specta]
#[allow(clippy::needless_pass_by_value)]
pub fn get_config(app: AppHandle) -> Config {
    Config::load(&app).unwrap()
}

#[tauri::command(async)]
#[specta::specta]
#[allow(clippy::needless_pass_by_value)]
pub fn save_config(app: AppHandle, config: Config) {
    config.save(&app).unwrap();
}
