use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc;
use yt_chat_tts::{
    request_audio_synthesis, subscribe_live_chat_messages, AudioDevice, ChatMessage, YTApiKey,
    YTVideoId,
};

#[derive(Deserialize)]
pub struct Config {
    pub video_id: YTVideoId,
    pub youtube_api_key: YTApiKey,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = envy::from_env::<Config>()?;

    let (_handle, audio_device) = AudioDevice::try_default()?;
    let audio_device = Arc::new(audio_device);

    // 読み上げるべきメッセージを通知するためのチャネル
    let (tx, mut rx) = mpsc::unbounded_channel::<ChatMessage>();

    let http_client = Arc::new(reqwest::Client::new());

    let http_client_cloned = http_client.clone();

    tokio::spawn(async move {
        subscribe_live_chat_messages(
            &http_client_cloned,
            &config.youtube_api_key,
            &config.video_id,
            &tx,
        )
        .await
    });

    while let Some(yt_chat_msg) = rx.recv().await {
        let http_client = Arc::clone(&http_client);

        let audio_device_cloned = Arc::clone(&audio_device);

        tokio::spawn(async move {
            // TODO: エラーをメインスレッドに通知したりロギングしたりする仕組みが必要
            let wav = request_audio_synthesis(&http_client, &yt_chat_msg.text)
                .await
                .unwrap();
            audio_device_cloned.append_wav(wav).unwrap();
        });
    }

    Ok(())
}
