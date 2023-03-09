use serde::Deserialize;
use tokio::sync::mpsc;
use yt_chat_tts::{
    display_live_chat_message_list_response, fetch_incoming_live_chat_messages,
    request_audio_synthesis, send_live_chat_messages, Config, YTChatMessage,
};

#[derive(Debug, Deserialize)]
struct YTLiveStreamingDetails {
    #[serde(rename(deserialize = "activeLiveChatId"))]
    active_live_chat_id: String,
}

#[derive(Debug, Deserialize)]
struct YTVideoInfo {
    #[serde(rename(deserialize = "liveStreamingDetails"))]
    live_streaming_details: YTLiveStreamingDetails,
}

#[derive(Debug, Deserialize)]
struct YTVideoListResponse {
    items: Vec<YTVideoInfo>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = envy::from_env::<Config>().expect("Failed to load config");

    let audio_device = yt_chat_tts::audio_device::AudioDevice::try_default()?;

    let (tx, mut rx) = mpsc::unbounded_channel::<YTChatMessage>();

    tokio::spawn(async move {
        let client = reqwest::Client::new();

        let youtube_api_key = config.youtube_api_key.as_str();

        let res = client
            .get("https://www.googleapis.com/youtube/v3/videos")
            .query(&[
                ("key", youtube_api_key),
                ("id", config.video_id.as_str()),
                ("part", "liveStreamingDetails"),
            ])
            .send()
            .await
            .expect("Failed to send request");

        let data = res
            .json::<YTVideoListResponse>()
            .await
            .expect("Failed to parse response");

        if data.items.len() != 1 {
            panic!("Unexpected number of items");
        }

        let live_chat_id = data.items[0]
            .live_streaming_details
            .active_live_chat_id
            .as_str();

        let mut next_page_token: Option<String> = None;

        loop {
            let data = fetch_incoming_live_chat_messages(
                &client,
                &config,
                live_chat_id,
                next_page_token.as_deref(),
            )
            .await;

            next_page_token = Some(data.next_page_token.clone());

            display_live_chat_message_list_response(&data);

            send_live_chat_messages(&tx, &data);

            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    while let Some(yt_chat_msg) = rx.recv().await {
        let wav = request_audio_synthesis(&yt_chat_msg.text).await?;
        audio_device.append_wav(wav)?;
    }

    Ok(())
}
