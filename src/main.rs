use std::io::Cursor;

use rodio::{Decoder, Source};
use tokio::sync::mpsc;
use yt_chat_tts::request_audio_synthesis;

#[derive(Debug)]
struct YouTubeChatMessage {
    text: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (_output_stream, output_stream_handle) = rodio::OutputStream::try_default()?;

    let sink = rodio::Sink::try_new(&output_stream_handle)?;

    let (tx, mut rx) = mpsc::unbounded_channel::<YouTubeChatMessage>();

    tokio::spawn(async move {
        tx.send(YouTubeChatMessage {
            text: "こんにちは".to_owned(),
        })
        .unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        tx.send(YouTubeChatMessage {
            text: "こんばんは".to_owned(),
        })
        .unwrap();
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
