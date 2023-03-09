use rodio::Source;

pub struct AudioDevice {
    _output_stream: rodio::OutputStream,
    _output_stream_handle: rodio::OutputStreamHandle,
    sink: rodio::Sink,
}

impl AudioDevice {
    pub fn try_default() -> anyhow::Result<Self> {
        let (output_stream, output_stream_handle) = rodio::OutputStream::try_default()?;
        let sink = rodio::Sink::try_new(&output_stream_handle)?;
        Ok(Self {
            _output_stream: output_stream,
            _output_stream_handle: output_stream_handle,
            sink,
        })
    }

    pub fn append_wav(&self, wav_to_play: Vec<u8>) -> anyhow::Result<()> {
        let decoder = rodio::Decoder::new(std::io::Cursor::new(wav_to_play))?;
        self.sink.append(decoder.convert_samples::<f32>());
        Ok(())
    }
}
