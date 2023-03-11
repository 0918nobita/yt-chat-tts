use serde::Deserialize;
use thiserror::Error;

/// API Key
#[derive(Deserialize)]
pub struct YTApiKey(String);

/// 動画 ID
#[derive(Deserialize)]
pub struct YTVideoId(String);

/// チャット ID
pub struct YTLiveChatId(pub String);

/// YouTube Data API v3 の呼び出し時に発生するエラー
#[derive(Debug, Error)]
pub enum YTError {
    #[error("Failed to request chat ID: {0}")]
    ChatIdRequest(reqwest::Error),

    #[error("Failed to decode chat ID: {0}")]
    ChatIdDecode(reqwest::Error),

    #[error("Failed to request live chat message list: {0}")]
    LiveChatMessageListRequest(reqwest::Error),

    #[error("Failed to decode live chat message list: {0}")]
    LiveChatMessageListDecode(reqwest::Error),
}

#[derive(Debug, Deserialize)]
pub struct YTLiveStreamingDetails {
    #[serde(rename(deserialize = "activeLiveChatId"))]
    pub active_live_chat_id: String,
}

#[derive(Debug, Deserialize)]
pub struct YTVideoInfo {
    #[serde(rename(deserialize = "liveStreamingDetails"))]
    pub live_streaming_details: YTLiveStreamingDetails,
}

#[derive(Debug, Deserialize)]
pub struct YTVideoListResponse {
    pub items: Vec<YTVideoInfo>,
}

/// 動画 ID をもとに生配信の情報付きの動画リストを取得する
pub async fn fetch_video_list_with_live_streaming_details(
    http_client: &reqwest::Client,
    api_key: &YTApiKey,
    video_id: &YTVideoId,
) -> Result<YTVideoListResponse, YTError> {
    http_client
        .get("https://www.googleapis.com/youtube/v3/videos")
        .query(&[
            ("key", api_key.0.as_str()),
            ("id", video_id.0.as_str()),
            ("part", "liveStreamingDetails"),
        ])
        .send()
        .await
        .map_err(YTError::ChatIdRequest)?
        .json::<YTVideoListResponse>()
        .await
        .map_err(YTError::ChatIdDecode)
}

/// チャットメッセージの投稿者に関する情報
#[derive(Debug, Deserialize)]
pub struct YTAuthorDetails {
    /// チャットメッセージの投稿者の表示名
    #[serde(rename(deserialize = "displayName"))]
    pub display_name: String,
}

/// チャットメッセージに関する追加情報
#[derive(Debug, Deserialize)]
pub struct YTSnippet {
    #[serde(rename(deserialize = "displayMessage"))]
    pub display_message: String,

    #[serde(rename(deserialize = "publishedAt"))]
    pub published_at: chrono::DateTime<chrono::Local>,
}

/// チャットメッセージ
#[derive(Debug, Deserialize)]
pub struct YTLiveChatMessage {
    /// 投稿者に関する情報
    #[serde(rename(deserialize = "authorDetails"))]
    pub author_details: YTAuthorDetails,

    /// メッセージの内容
    pub snippet: YTSnippet,
}

/// チャットメッセージのリストを取得する API のレスポンス
#[derive(Debug, Deserialize)]
pub struct YTLiveChatMessageListResponse {
    /// チャットメッセージのリスト
    pub items: Vec<YTLiveChatMessage>,

    /// 次のページを取得するために使用するトークン
    #[serde(rename(deserialize = "nextPageToken"))]
    pub next_page_token: String,
}

/// ライブ配信のチャットメッセージのリストを取得する
pub async fn fetch_live_chat_messages(
    http_client: &reqwest::Client,
    api_key: &YTApiKey,
    live_chat_id: &YTLiveChatId,
    page_token: Option<&str>,
) -> Result<YTLiveChatMessageListResponse, YTError> {
    let mut query: Vec<(&str, &str)> = vec![
        ("key", &api_key.0),
        ("liveChatId", &live_chat_id.0),
        ("part", "id,snippet,authorDetails"),
    ];

    if let Some(page_token) = page_token {
        query.push(("pageToken", page_token));
    }

    http_client
        .get("https://www.googleapis.com/youtube/v3/liveChat/messages")
        .query(&query)
        .send()
        .await
        .map_err(YTError::LiveChatMessageListRequest)?
        .json::<YTLiveChatMessageListResponse>()
        .await
        .map_err(YTError::LiveChatMessageListDecode)
}
