use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};
use actix_web_actors::ws;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

mod websocket;

#[derive(Deserialize, Serialize, Debug)]
struct HealthCheckResponse {
    status: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let response = HealthCheckResponse { status: "ok".to_string() };
    HttpResponse::Ok().json(response)
}


/// Define http actor
async fn ws_route(req: web::HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(websocket::MyWebSocket::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");

    HttpServer::new(|| {
        App::new()
            .service(health_check())
            .route("/ws/", web::get().to(ws_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
