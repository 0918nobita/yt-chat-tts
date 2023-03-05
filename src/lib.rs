mod sound_player;
mod synthesis;

use actix::Actor;
use sound_player::{SoundPlayerActor, SoundPlayerRequest};
use synthesis::{SynthesisActor, SynthesisRequest};

pub fn event_loop(
    output_stream: rodio::OutputStream,
    output_stream_handle: rodio::OutputStreamHandle,
    sink: rodio::Sink,
) -> std::io::Result<()> {
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
            .expect("Failed to send message to synthesis actor")
            .expect("Failed to synthesize sound");

        sound_player_addr
            .send(SoundPlayerRequest { wav })
            .await
            .expect("Failed to send message to sound_player actor")
            .expect("Failed to play sound");
    });

    system.run()
}
