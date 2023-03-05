use actix::{Actor, Context as ActixContext, Handler, Message, ResponseFuture};
use anyhow::Context;
use serde_json::Value as JsonValue;

#[derive(Message)]
#[rtype(result = "anyhow::Result<Vec<u8>>")]
pub struct SynthesisRequest {
    pub text: String,
}

pub struct SynthesisActor;

impl Actor for SynthesisActor {
    type Context = ActixContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("SynthesisActor started");
    }
}

impl Handler<SynthesisRequest> for SynthesisActor {
    type Result = ResponseFuture<anyhow::Result<Vec<u8>>>;

    fn handle(&mut self, msg: SynthesisRequest, _ctx: &mut Self::Context) -> Self::Result {
        Box::pin(async move {
            let SynthesisRequest { text } = msg;

            let client = reqwest::Client::new();

            let res = client
                .post("http://localhost:50021/audio_query")
                .query(&[("text", text.as_str()), ("speaker", "1")])
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
            Ok(out_wav.to_vec())
        })
    }
}
