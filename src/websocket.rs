use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Define message that websocket will receive and respond to.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub content: String,
}

/// Define websocket actor
pub struct WebSocketActor {
    id: Uuid,
}

impl WebSocketActor {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
        }
    }
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection opened: {}", self.id);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection closed: {}", self.id);
    }
}

/// Handle messages from the websocket client
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("Received message: {}", text);

                // Echo the message back to the client
                ctx.text(format!("Echo: {}", text))
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
