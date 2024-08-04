use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct DeeplXResult {
    pub code: i64,
    pub id: i64,
    pub data: String,
    pub alternatives: Vec<Alternative>,
}

#[derive(Deserialize, Debug)]
pub struct Alternative {
    pub text: String,
}

pub async fn translate(
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
