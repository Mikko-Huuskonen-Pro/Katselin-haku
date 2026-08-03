//! Katselin Android PoC 3 — actix-web + tokio on localhost.
//! No Meilisearch, no TLS, no compression.

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

const ADDR: &str = "127.0.0.1:17701";

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status":"ok"}"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    eprintln!("Katselin PoC3: listening on http://{ADDR}");
    HttpServer::new(|| App::new().service(health))
        .bind(ADDR)?
        .workers(2)
        .run()
        .await
}
