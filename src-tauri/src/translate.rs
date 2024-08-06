use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::http::{HeaderMap, HeaderValue};

#[derive(Deserialize, Debug)]
pub struct DeeplXResult {
    pub code: i64,
    pub id: i64,
    pub data: String,
    pub alternatives: Vec<Alternative>,
}
#[derive(Serialize, Debug)]
struct Lang<'a> {
    pub source_lang_user_selected: &'a str,
    pub target_lang: &'a str,
}
#[derive(Serialize, Debug)]
struct CommonJobParams<'a> {
    pub was_spoken: bool,
    pub transcribe_as: &'a str,
}
#[allow(clippy::struct_field_names)]
#[derive(Serialize, Debug)]
struct Params<'a> {
    pub texts: Vec<Text<'a>>,
    pub splitting: &'a str,
    pub lang: Lang<'a>,
    pub timestamp: i64,
    #[serde(rename = "commonJobParams")]
    pub common_job_params: CommonJobParams<'a>,
}
#[derive(Deserialize, Serialize, Debug)]
struct Text<'a> {
    pub text: &'a str,
    #[serde(rename = "requestAlternatives")]
    pub request_alternatives: i32,
}
#[derive(Serialize, Debug)]
struct PostData<'a> {
    pub jsonrpc: &'a str,
    pub method: &'a str,
    pub id: i64,
    pub params: Params<'a>,
}
#[derive(Deserialize, Debug)]
pub struct DeepLResponse {
    pub jsonrpc: String,
    pub id: i64,
    pub result: DeeplResult,
}
#[derive(Deserialize, Debug)]
pub struct DeeplResult {
    pub texts: Vec<TranslatedText>,
    pub lang: String,
    pub lang_is_confident: bool,
    #[serde(rename = "detectedLanguages")]
    pub detected_languages: HashMap<String, f64>,
}
#[derive(Deserialize, Debug)]
pub struct TranslatedText {
    pub alternatives: Vec<Alternative>,
    pub text: String,
}
#[derive(Deserialize, Debug)]
pub struct Alternative {
    pub text: String,
}

pub async fn with_api_key(
    from: &str,
    to: &str,
    content: &str,
    api_key: &str,
) -> anyhow::Result<DeeplXResult> {
    let post_str = json!({
      "text": content,
      "source_lang": from,
      "target_lang": to
    })
    .to_string();

    let resp = reqwest::Client::new()
        .post(format!("https://api.deeplx.org/{api_key}/translate"))
        .body(post_str)
        .send()
        .await?;
    if resp.status() != 200 {
        let status = resp.status();
        let body = resp.text().await?;
        return Err(anyhow::anyhow!(
            "Failed to get response from DeepL\nstatus code: {status}\nbody: {body}"
        ));
    }
    let deepl_x_res = resp.json::<DeeplXResult>().await?;
    Ok(deepl_x_res)
}

pub async fn without_api_key(from: &str, to: &str, content: &str) -> anyhow::Result<DeeplResult> {
    let id = get_random_number() + 1;
    let post_data = create_post_data(id, from, to, content);

    let mut post_str = serde_json::to_string(&post_data)?;
    if (id + 5) % 29 == 0 || (id + 3) % 13 == 0 {
        post_str = post_str.replace("\"method\":\"", "\"method\" : \"");
    } else {
        post_str = post_str.replace("\"method\":\"", "\"method\": \"");
    }

    let mut headers = HeaderMap::with_capacity(10);
    let default_headers = [
        ("Content-Type", "application/json"),
        ("Accept", "*/*"),
        ("x-app-os-name", "iOS"),
        ("x-app-os-version", "16.3.0"),
        ("Accept-Language", "en-US,en;q=0.9"),
        ("x-app-device", "iPhone13,2"),
        ("User-Agent", "DeepL-iOS/2.9.1 iOS 16.3.0 (iPhone13,2)"),
        ("x-app-build", "510265"),
        ("x-app-version", "2.9.1"),
        ("Connection", "keep-alive"),
    ];

    for &(key, value) in &default_headers {
        headers.insert(key, HeaderValue::from_static(value));
    }

    let resp = reqwest::Client::new()
        .post("https://www2.deepl.com/jsonrpc")
        .headers(headers)
        .body(post_str)
        .send()
        .await?;
    if resp.status() != 200 {
        let status = resp.status();
        let body = resp.text().await?;
        return Err(anyhow::anyhow!(
            "Failed to get response from DeepL\nstatus code: {status}\nbody: {body}"
        ));
    }

    let deepl_resp = resp.json::<DeepLResponse>().await?;
    Ok(deepl_resp.result)
}

#[allow(clippy::cast_possible_wrap)]
fn get_i_count(translate_text: &str) -> i64 {
    translate_text.matches('i').count() as i64
}

fn get_random_number() -> i64 {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..99999) + 8_300_000) * 1000
}

#[allow(clippy::cast_possible_truncation)]
fn get_time_stamp(i_count: i64) -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ts = since_the_epoch.as_millis() as i64;

    if i_count != 0 {
        ts - ts % (i_count + 1) + (i_count + 1)
    } else {
        ts
    }
}

fn create_post_data<'a>(id: i64, from: &'a str, to: &'a str, content: &'a str) -> PostData<'a> {
    PostData {
        jsonrpc: "2.0",
        method: "LMT_handle_texts",
        id,
        params: Params {
            texts: vec![Text {
                text: content,
                request_alternatives: 3,
            }],
            splitting: "newlines",
            lang: Lang {
                source_lang_user_selected: from,
                target_lang: to,
            },
            timestamp: get_time_stamp(get_i_count(content)),
            common_job_params: CommonJobParams {
                was_spoken: false,
                transcribe_as: "",
            },
        },
    }
}
