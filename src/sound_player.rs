use actix::{Actor, Context as ActixContext, Handler, Message};
use rodio::{Sink, Source};

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct SoundPlayerRequest {
    pub wav: Vec<u8>,
}

pub struct SoundPlayerActor {
    pub output_stream: rodio::OutputStream,
    pub output_stream_handle: rodio::OutputStreamHandle,
    pub sink: Sink,
}

impl Actor for SoundPlayerActor {
    type Context = ActixContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("SoundPlayerActor started");
    }
}

impl Handler<SoundPlayerRequest> for SoundPlayerActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: SoundPlayerRequest, _ctx: &mut Self::Context) -> Self::Result {
        let SoundPlayerRequest { wav } = msg;

        let source = rodio::Decoder::new(std::io::Cursor::new(wav))?;

        self.sink.append(source.convert_samples::<f32>());

        self.sink.sleep_until_end();
        Ok(())
    }
}
