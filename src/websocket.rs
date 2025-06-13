use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Define message that WebSocket actor can receive
#[derive(Deserialize, Serialize, Debug)]
pub struct ClientMessage {
    pub content: String,
}

/// Define message that WebSocket actor can send
#[derive(Deserialize, Serialize, Debug)]
pub struct ServerMessage {
    pub id: Uuid,
    pub content: String,
}

/// Define WebSocket actor
pub struct WebSocketActor;

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection opened");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection closed");
    }
}

/// Handle incoming messages from the client
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(
        &mut self, 
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg)
            },
            Ok(ws::Message::Text(text)) => {
                println!("Received message: {:?}", text);

                // Deserialize the incoming JSON
                let client_message: Result<ClientMessage, serde_json::Error> = serde_json::from_str(&text);

                match client_message {
                    Ok(client_msg) => {
                        // Process the message and send a response
                        let server_message = ServerMessage {
                            id: Uuid::new_v4(),
                            content: format!("You said: {}", client_msg.content),
                        };

                        // Serialize the response to JSON
                        match serde_json::to_string(&server_message) {
                            Ok(response_text) => {
                                ctx.text(response_text)
                            },
                            Err(e) => {
                                eprintln!("Failed to serialize server message: {}", e);
                                ctx.text("Error serializing server message");
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to deserialize client message: {}", e);
                        ctx.text("Invalid JSON");
                    }
                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}