use rust_i18n::t;
use tauri::AppHandle;

use crate::config::Config;

#[tauri::command(async)]
#[specta::specta]
pub async fn greet(app: AppHandle, name: &str) -> Result<String, String> {
    let config = Config::load(&app).unwrap();
    let api_key = config.api_key.unwrap();
    match crate::translate_with_api_key::translate("auto", "en", "今天的天气真不错", &api_key).await
    {
        Ok(deepl_result) => {
            let translated_text = &deepl_result.data;
            let mut text = translated_text.clone();
            if !deepl_result.alternatives.is_empty() {
                let alternative_i18n = t!("translate.alternative");
                text += format!("\n\n====={alternative_i18n}=====\n").as_str();
                for alternative in &deepl_result.alternatives {
                    text += format!("{}\n", alternative.text).as_str();
                }
            }
            println!("translated text: {}", text);
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
