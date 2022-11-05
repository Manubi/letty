use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::net::TcpListener;
// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // No .await here!
    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn subscribe(form: web::Form<FormData>) -> String {
    format!("Welcome {} with email {}", form.name, form.email)
}
