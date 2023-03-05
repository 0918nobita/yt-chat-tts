use actix::{Actor, Context};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub struct YouTubeChatMessage {
    pub text: String,
}

/// YouTube ライブ配信の新着コメントを取得するアクター
pub struct YouTubeChatActor {
    pub sender: UnboundedSender<YouTubeChatMessage>,
}

impl Actor for YouTubeChatActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("YoutubeChatActor started");

        self.sender
            .send(YouTubeChatMessage {
                text: "新着メッセージ1".to_owned(),
            })
            .unwrap();

        self.sender
            .send(YouTubeChatMessage {
                text: "新着メッセージ2".to_owned(),
            })
            .unwrap();
    }
}
