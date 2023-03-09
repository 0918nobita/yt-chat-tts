use std::sync::Arc;
use tokio::sync::mpsc;
use yt_chat_tts::{
    request_audio_synthesis, subscribe_live_chat_messages, AudioDevice, ChatMessage, Config,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = envy::from_env::<Config>()?;

    let audio_device = AudioDevice::try_default()?;

    // 読み上げるべきメッセージを通知するためのチャネル
    let (tx, mut rx) = mpsc::unbounded_channel::<ChatMessage>();

    let http_client = Arc::new(reqwest::Client::new());

    let http_client_cloned = http_client.clone();

    tokio::spawn(
        async move { subscribe_live_chat_messages(&*http_client_cloned, &config, &tx).await },
    );

    while let Some(yt_chat_msg) = rx.recv().await {
        let wav = request_audio_synthesis(&*http_client, &yt_chat_msg.text).await?;
        audio_device.append_wav(wav)?;
    }

    Ok(())
}
