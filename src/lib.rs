pub mod audio_device;

use anyhow::Context;
use serde::Deserialize;
use serde_json::Value as JsonValue;

#[derive(Deserialize)]
pub struct Config {
    pub video_id: String,
    pub youtube_api_key: String,
}

/// VOICEVOX で音声合成する
pub async fn request_audio_synthesis(text: &str) -> anyhow::Result<Vec<u8>> {
    let client = reqwest::Client::new();

    let res = client
        .post("http://localhost:50021/audio_query")
        .query(&[("text", text), ("speaker", "1")])
        .send()
        .await?;

    let bytes = res.bytes().await?;
    let bytes = bytes.as_ref();
    let json_str = std::str::from_utf8(bytes)?;

    let query_object: JsonValue = serde_json::from_str(json_str)?;
    let mut query_object = query_object
        .as_object()
        .context("invalid query format")?
        .clone();

    query_object.insert(
        "volumeScale".to_owned(),
        JsonValue::Number(serde_json::Number::from_f64(2.0).unwrap()),
    );

    let query_object = JsonValue::Object(query_object).to_string();

    let res = client
        .post("http://localhost:50021/synthesis")
        .header("Content-Type", "application/json")
        .query(&[("speaker", "1")])
        .body(query_object)
        .send()
        .await?;

    let out_wav = res.bytes().await?;
    Ok(out_wav.to_vec())
}

#[derive(Debug, Deserialize)]
struct YTAuthorDetails {
    #[serde(rename(deserialize = "displayName"))]
    display_name: String,
}

#[derive(Debug, Deserialize)]
struct YTSnippet {
    #[serde(rename(deserialize = "displayMessage"))]
    display_message: String,
}

#[derive(Debug, Deserialize)]
struct YTLiveChatMessage {
    #[serde(rename(deserialize = "authorDetails"))]
    author_details: YTAuthorDetails,

    snippet: YTSnippet,
}

#[derive(Debug, Deserialize)]
pub struct YTLiveChatMessageListResponse {
    items: Vec<YTLiveChatMessage>,

    #[serde(rename(deserialize = "nextPageToken"))]
    pub next_page_token: String,
}

pub async fn fetch_incoming_live_chat_messages(
    client: &reqwest::Client,
    config: &Config,
    live_chat_id: &str,
    page_token: Option<&str>,
) -> YTLiveChatMessageListResponse {
    let mut query: Vec<(&str, &str)> = vec![
        ("key", config.youtube_api_key.as_str()),
        ("liveChatId", live_chat_id),
        ("part", "id,snippet,authorDetails"),
    ];

    if let Some(page_token) = page_token {
        query.push(("pageToken", page_token));
    }

    let res = client
        .get("https://www.googleapis.com/youtube/v3/liveChat/messages")
        .query(&query)
        .send()
        .await
        .expect("Failed to send request");

    res.json::<YTLiveChatMessageListResponse>()
        .await
        .expect("Failed to deserialize response")
}

pub fn display_live_chat_message_list_response(response: &YTLiveChatMessageListResponse) {
    println!("-----");

    for item in &response.items {
        println!(
            "{}: {}",
            item.author_details.display_name.as_str(),
            item.snippet.display_message.as_str()
        )
    }
}

#[derive(Debug)]
pub struct YTChatMessage {
    pub text: String,
}

pub fn send_live_chat_messages(
    tx: &tokio::sync::mpsc::UnboundedSender<YTChatMessage>,
    response: &YTLiveChatMessageListResponse,
) {
    for item in &response.items {
        let author = item.author_details.display_name.as_str();

        let text = item.snippet.display_message.as_str();

        let yt_chat_message = YTChatMessage {
            text: format!("{} さん、{}", author, text),
        };

        tx.send(yt_chat_message).unwrap();
    }
}
