use actix::Actor;
use anyhow::Context;
use yt_chat_tts::{
    sound_player::{SoundPlayerActor, SoundPlayerRequest},
    synthesis::{SynthesisActor, SynthesisRequest},
};

fn main() -> anyhow::Result<()> {
    let (output_stream, output_stream_handle) = rodio::OutputStream::try_default()?;

    let sink = rodio::Sink::try_new(&output_stream_handle)?;

    let system = actix::System::new();

    system.block_on(async {
        let synthesis_addr = SynthesisActor.start();

        let sound_player_addr = (SoundPlayerActor {
            output_stream,
            output_stream_handle,
            sink,
        })
        .start();

        let wav = synthesis_addr
            .send(SynthesisRequest {
                text: "こんにちは".to_owned(),
            })
            .await
            .expect("Failed to send message")
            .expect("Failed to synthesize sound");

        sound_player_addr
            .send(SoundPlayerRequest { wav })
            .await
            .expect("Failed to send message to sound_player actor")
            .expect("Failed to play sound");
    });

    system.run().context("Actix system stopped")
}
