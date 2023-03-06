use std::io::Cursor;

use rodio::{Decoder, Source};
use serde::Deserialize;
use tokio::sync::mpsc;
use yt_chat_tts::request_audio_synthesis;

#[derive(Debug)]
struct YouTubeChatMessage {
    text: String,
}

#[derive(Deserialize)]
struct Config {
    video_id: String,
    youtube_api_key: String,
}

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

    let (_output_stream, output_stream_handle) = rodio::OutputStream::try_default()?;

    let sink = rodio::Sink::try_new(&output_stream_handle)?;

    let (tx, mut rx) = mpsc::unbounded_channel::<YouTubeChatMessage>();

    tokio::spawn(async move {
        // tx.send(YouTubeChatMessage {
        //     text: "こんにちは".to_owned(),
        // })
        // .unwrap();

        // tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        // tx.send(YouTubeChatMessage {
        //     text: "こんばんは".to_owned(),
        // })
        // .unwrap();

        let client = reqwest::Client::new();

        let res = client
            .get("https://www.googleapis.com/youtube/v3/videos")
            .query(&[
                ("key", config.youtube_api_key.as_str()),
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

        let live_chat_id = &data.items[0].live_streaming_details.active_live_chat_id;
        println!("LiveChatID: {}", live_chat_id);
    });

    while let Some(yt_chat_msg) = rx.recv().await {
        println!("Got {:?}", yt_chat_msg);

        let wav = request_audio_synthesis(&yt_chat_msg.text).await?;
        println!("Audio synthesis for {:?} succeeded", yt_chat_msg);

        let source = Decoder::new(Cursor::new(wav))?;
        sink.append(source.convert_samples::<f32>());

        sink.sleep_until_end();
    }

    Ok(())
}
