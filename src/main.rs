use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;
use futures::StreamExt;


// Define WebSocket message format
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
enum WsMessage {
    CodeUpdate { code: String },
    CursorMove { line: usize, character: usize },
    UserJoin { user_id: Uuid },
    UserLeave { user_id: Uuid },
    UserList { user_ids: Vec<Uuid> }, // Server -> Client
    Ack, // Acknowledge receipt of message (optional)
    Error { message: String } // Indicate errors
}

// WebSocket actor
struct MyWebSocketActor {
    id: Uuid,
    room_id: Uuid, // Add room_id to the actor
    server_addr: Addr<WsServer>,
}

impl Actor for MyWebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Register self with the server.
        let addr = ctx.address();
        self.server_addr
            .do_send(Connect { addr: addr.recipient(), user_id: self.id, room_id: self.room_id });

        println!("WebSocket Actor started");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
         // Deregister self with the server.
         self.server_addr.do_send(Disconnect { user_id: self.id, room_id: self.room_id });
        println!("WebSocket Actor disconnected");
    }
}


// Define handler for websocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocketActor {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                // Deserialize the message
                match serde_json::from_str::<WsMessage>(&text) {
                    Ok(ws_message) => {
                        // Handle the message based on its type
                        match ws_message {
                            WsMessage::CodeUpdate { code } => {
                                // Broadcast the code update to other clients in the room
                                self.server_addr.do_send(ServerMessage { room_id: self.room_id, message: WsMessage::CodeUpdate { code } });
                            }
                            WsMessage::CursorMove { line, character } => {
                                // Broadcast the cursor movement to other clients in the room
                                self.server_addr.do_send(ServerMessage { room_id: self.room_id, message: WsMessage::CursorMove { line, character } });
                            }
                            _ => println!("Unhandled message: {:?}", ws_message),
                        }
                    }
                    Err(e) => {
                        println!("Failed to deserialize message: {:?}, Error: {}", text, e);
                        // Send an error message back to the client
                        let error_message = WsMessage::Error { message: format!("Failed to parse message: {}", e) };
                        let serialized_error = serde_json::to_string(&error_message).unwrap(); // Safe to unwrap as we control the enum.
                        ctx.text(serialized_error);
                    }
                }
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



// Message to register a new session
#[derive(Message)]
#[rtype(result = "()")]
struct Connect {
    addr: Recipient<WsMessage>,
    user_id: Uuid,
    room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Disconnect {
    user_id: Uuid,
    room_id: Uuid,
}


// Message for broadcasting to a room
#[derive(Message, Debug)]
#[rtype(result = "()")]
struct ServerMessage {
    room_id: Uuid,
    message: WsMessage,
}


// WebSocket server actor
struct WsServer {
    // Using a HashMap to store users per room. Key is room_id, value is a Vec of (user_id, address).
    rooms: std::collections::HashMap<Uuid, Vec<(Uuid, Recipient<WsMessage>)>>,
}

impl Actor for WsServer {
    type Context = Context<Self>;
}

impl WsServer {
    fn new() -> Self {
        WsServer {
            rooms: std::collections::HashMap::new(),
        }
    }

    fn broadcast(&self, room_id: Uuid, message: WsMessage) {
        if let Some(users) = self.rooms.get(&room_id) {
            for (user_id, addr) in users {
                println!("Sending message to user: {}, Room: {}", user_id, room_id);
                addr.do_send(message.clone()); // Clone the message for each recipient.
            }
        }
    }
}


// Handle Connect message
impl Handler<Connect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) {
        let Connect { addr, user_id, room_id } = msg;
        println!("New user: {} joined room: {}", user_id, room_id);

        // Add the user to the room.
        self.rooms.entry(room_id).or_insert(Vec::new()).push((user_id, addr));

        // Notify existing users about the new user (optional).
        self.broadcast(room_id, WsMessage::UserJoin { user_id });

        // Send the new user a list of existing users.  Construct the user list *after* adding the new user to the room.  This ensures the new user is included. 
        if let Some(users) = self.rooms.get(&room_id) {
            let user_ids: Vec<Uuid> = users.iter().map(|(id, _)| *id).collect();
            addr.do_send(WsMessage::UserList { user_ids });
        }
    }
}

// Handle Disconnect message
impl Handler<Disconnect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) {
        let Disconnect { user_id, room_id } = msg;
        println!("User: {} left room: {}", user_id, room_id);

        // Remove the user from the room.
        if let Some(users) = self.rooms.get_mut(&room_id) {
            users.retain(|(id, _)| *id != user_id);
            if users.is_empty() {
                self.rooms.remove(&room_id);
            }
        }

        // Notify remaining users about the leaving user (optional).
        self.broadcast(room_id, WsMessage::UserLeave { user_id });
    }
}


// Handle ServerMessage message
impl Handler<ServerMessage> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, _ctx: &mut Self::Context) {
        self.broadcast(msg.room_id, msg.message);
    }
}


// Define websocket handler
async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<WsServer>>,
    room_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let room_id = room_id.into_inner();
    let user_id = Uuid::new_v4(); // Generate a unique user ID
    ws::start(
        MyWebSocketActor {
            id: user_id,
            room_id: room_id,
            server_addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the WebSocket server actor
    let server = WsServer::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone())) // Pass the server address to the application state
            .route("/ws/{room_id}", web::get().to(ws_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
