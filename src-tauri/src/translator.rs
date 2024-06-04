use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;
use reqwest::StatusCode;
use serde_json::json;

pub struct Translator {
    http_client: reqwest::Client,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn translate(&self, from: &str, to: &str, content: &str) -> anyhow::Result<String> {
        let payload = json!({
            "query": content,
            "from": from,
            "to": to,
            "reference": "",
            "corpusIds": [],
            "qcSettings": ["1","2","3","4","5","6","7","8","9","10","11"],
            "needPhonetic": true,
            "domain": "common",
            "milliTimestamp": SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
        });

        println!("{:?}", payload);

        let resp = self
            .http_client
            .post("https://fanyi.baidu.com/ait/text/translate")
            .json(&payload)
            .send()
            .await?;
        let status_code = resp.status();
        let resp_body = resp.text().await?;

        println!("{:?}", resp_body);

        if status_code != StatusCode::OK {
            return Err(anyhow::anyhow!(
                "Request failed with status code: {}\nresponse body:\n{}",
                status_code,
                resp_body
            ));
        }

        // TODO: 可以用lazy优化
        let re = Regex::new(r#"\{[^}]*"data":\{"event":"Translating".*\}\}"#)?;
        let data = re
            .find(&resp_body)
            .ok_or(anyhow::anyhow!(
                "Regex not match, response body:\n{}",
                resp_body
            ))?
            .as_str();
        let data: serde_json::Value = serde_json::from_str(data)?;
        println!("{data}");

        let list = data["data"]["list"]
            .as_array()
            .ok_or(anyhow::anyhow!("data.list not found in json:\n{}", data))?;

        let mut result = "".to_string();
        for item in list {
            result += format!(
                "{}\n",
                item["dst"]
                    .as_str()
                    .ok_or(anyhow::anyhow!("dst not found in json:\n{}", item))?
            )
            .as_str();
        }

        Ok(result)
    }

    pub async fn detect_language(&self, content: &str) -> anyhow::Result<String> {
        let payload = json!({
            "query": content
        });

        let resp = self
            .http_client
            .post("https://fanyi.baidu.com/langdetect")
            .form(&payload)
            .send()
            .await?;

        let status_code = resp.status();
        let resp_body = resp.text().await?;

        if status_code != StatusCode::OK {
            return Err(anyhow::anyhow!(
                "Request failed with status code: {}\nresponse body:\n{}",
                status_code,
                resp_body
            ));
        }

        let data: serde_json::Value = serde_json::from_str(&resp_body)?;

        let lan = data["lan"]
            .as_str()
            .ok_or(anyhow::anyhow!("lan not found in json:\n{}", data))?
            .to_string();

        Ok(lan)
    }
}
