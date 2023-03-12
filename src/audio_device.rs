use anyhow::Context;
use rodio::Source;

pub struct AudioDevice {
    sink: rodio::Sink,
}

pub struct AudioDeviceHandle {
    _output_stream: rodio::OutputStream,
    _output_stream_handle: rodio::OutputStreamHandle,
}

impl AudioDevice {
    #[tracing::instrument(name = "Try to detect default audio device")]
    pub fn try_default() -> anyhow::Result<(AudioDeviceHandle, Self)> {
        let (output_stream, output_stream_handle) =
            rodio::OutputStream::try_default().context("Failed to detect default audio device")?;

        let sink =
            rodio::Sink::try_new(&output_stream_handle).context("Failed to create audio sink")?;

        Ok((
            AudioDeviceHandle {
                _output_stream: output_stream,
                _output_stream_handle: output_stream_handle,
            },
            Self { sink },
        ))
    }

    #[tracing::instrument(
        name = "Append wave data to the sink of default audio device",
        skip(self, wav_to_play)
    )]
    pub fn append_wav(&self, wav_to_play: Vec<u8>) -> anyhow::Result<()> {
        let decoder = rodio::Decoder::new(std::io::Cursor::new(wav_to_play))?;

        self.sink.append(decoder.convert_samples::<f32>());
        Ok(())
    }
}
