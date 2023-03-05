use anyhow::Context;
use yt_chat_tts::event_loop;

fn main() -> anyhow::Result<()> {
    let (output_stream, output_stream_handle) = rodio::OutputStream::try_default()?;

    let sink = rodio::Sink::try_new(&output_stream_handle)?;

    event_loop(
        output_stream,
        output_stream_handle,
        sink
    ).context("Event loop stopped unexpectedly")
}
