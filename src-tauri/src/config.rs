use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Config {
    pub from: String,
    pub to: String,
    #[serde(rename = "apiKey")]
    pub api_key: Option<String>,
    /// BCP-47 language tag
    pub local: String,
}

impl Config {
    pub fn load(app: &AppHandle) -> anyhow::Result<Self> {
        let resource_dir = app.path().resource_dir()?;
        let config_path = resource_dir.join("config.json");
        // FIXME: 应该判断文件是否存在，如果不存在才创建默认配置，而不是直接创建默认配置
        let sys_locale = sys_locale::get_locale().unwrap_or("en-US".to_owned());
        let default_to = if sys_locale.starts_with("zh") {
            "zh"
        } else {
            "en"
        }
        .to_owned();
        let default_config = Config {
            from: "auto".to_owned(),
            to: default_to,
            api_key: None,
            local: sys_locale,
        };
        // 如果配置文件存在且能够解析，则使用配置文件中的配置，否则使用默认配置
        let config = if config_path.exists() {
            let config_string = std::fs::read_to_string(config_path)?;
            serde_json::from_str(&config_string).unwrap_or(default_config)
        } else {
            default_config
        };
        config.save(app)?;

        Ok(config)
    }

    pub fn save(&self, app: &AppHandle) -> anyhow::Result<()> {
        let resource_dir = app.path().resource_dir()?;
        let config_path = resource_dir.join("config.json");
        let config_string = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, config_string)?;
        Ok(())
    }
}
