use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
use actix_web_actors::ws;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

mod websocket;

#[derive(Deserialize, Serialize)]
struct HealthCheckResponse {
    status: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let response = HealthCheckResponse { status: "ok".to_string() };
    HttpResponse::Ok().json(response)
}


/// Define HTTP route for websocket handshake
#[get("/ws/")]
async fn websocket_route(req: web::HttpRequest, stream: web::Payload) -> impl Responder {
    ws::start(websocket::MyWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(websocket_route)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
