mod youtube_api;

use self::youtube_api::{
    fetch_live_chat_messages, fetch_video_list_with_live_streaming_details, YTLiveChatId,
};
use thiserror::Error;
use tokio::sync::mpsc;

pub use self::youtube_api::{YTApiKey, YTVideoId};

#[derive(Debug)]
pub struct ChatMessage {
    pub text: String,
}

#[derive(Debug, Error)]
pub enum LiveChatMessageSubscriptionError {
    #[error("Chat ID not found")]
    ChatIdNotFound,

    #[error("Multiple chat IDs found")]
    MultipleChatIdsFound,

    #[error(transparent)]
    ApiError(#[from] youtube_api::YTError),
}

#[tracing::instrument(
    name = "Subscribe live chat messages",
    skip(http_client, youtube_api_key, tx)
)]
pub async fn subscribe_live_chat_messages(
    http_client: &reqwest::Client,
    youtube_api_key: &YTApiKey,
    video_id: &YTVideoId,
    tx: &mpsc::UnboundedSender<ChatMessage>,
    published_after: &chrono::DateTime<chrono::Local>,
) -> Result<(), LiveChatMessageSubscriptionError> {
    let video_list_res =
        fetch_video_list_with_live_streaming_details(http_client, youtube_api_key, video_id)
            .await?;

    if video_list_res.items.len() > 1 {
        return Err(LiveChatMessageSubscriptionError::MultipleChatIdsFound);
    }

    let Some(video_info) = video_list_res.items.get(0) else {
        return Err(LiveChatMessageSubscriptionError::ChatIdNotFound);
    };

    let live_chat_id = YTLiveChatId(
        video_info
            .live_streaming_details
            .active_live_chat_id
            .clone(),
    );

    let mut page_token: Option<String> = None;

    loop {
        let res = fetch_live_chat_messages(
            http_client,
            youtube_api_key,
            &live_chat_id,
            page_token.as_deref(),
        )
        .await?;

        page_token = Some(res.next_page_token.clone());

        let incoming_live_chat_messages = res
            .items
            .iter()
            .filter(|item| item.snippet.published_at >= *published_after)
            .collect::<Vec<_>>();

        for item in &incoming_live_chat_messages {
            let author = item.author_details.display_name.as_str();

            let text = item.snippet.display_message.as_str();

            let yt_chat_message = ChatMessage {
                text: format!("{}?????????{}", author, text),
            };

            tx.send(yt_chat_message).unwrap();
        }

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}
