// src/websocket.rs

use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use futures::future::ready;

/// Define message that `WsActor` can handle
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub text: String,
}

/// Define http actor
pub struct WsActor;

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection established");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection closed");
    }
}

/// Handle incoming messages from the client
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg)
            }
            Ok(ws::Message::Text(text)) => {
                println!("Received message: {}", text);

                // Example: Echo the message back with a UUID
                let message = Message {
                    id: Uuid::new_v4(),
                    text: text.to_string(),
                };
                let json_message = serde_json::to_string(&message).unwrap();

                ctx.text(json_message)
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

// Define the handler for the `Message` struct (example)
impl Handler<Message> for WsActor {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        let json_message = serde_json::to_string(&msg).unwrap();
        ctx.text(json_message);
        Ok(())
    }
}