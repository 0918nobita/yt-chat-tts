use anyhow::Context;
use rodio::{source::Source, Decoder, OutputStream};
use serde_json::Value as JsonValue;
use std::io::Cursor;

pub async fn read_text_using_voicevox(text: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let res = client
        .post("http://localhost:50021/audio_query")
        .query(&[("text", text), ("speaker", "1")])
        .send()
        .await?;

    let bytes = res.bytes().await?;
    let bytes = bytes.as_ref();
    let json_str = std::str::from_utf8(bytes)?;

    let query_object: JsonValue = serde_json::from_str(json_str)?;
    let mut query_object = query_object
        .as_object()
        .context("invalid query format")?
        .clone();

    query_object.insert(
        "volumeScale".to_owned(),
        JsonValue::Number(serde_json::Number::from_f64(2.0).unwrap()),
    );

    let query_object = JsonValue::Object(query_object).to_string();

    let res = client
        .post("http://localhost:50021/synthesis")
        .header("Content-Type", "application/json")
        .query(&[("speaker", "1")])
        .body(query_object)
        .send()
        .await?;

    let out_wav = res.bytes().await?;
    let out_wav = out_wav.to_vec();

    let (_stream, stream_handle) = OutputStream::try_default()?;

    let source = Decoder::new(Cursor::new(out_wav))?;

    stream_handle.play_raw(source.convert_samples())?;

    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}
