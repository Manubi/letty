use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> String {
    format!("Welcome {} with email {}", form.name, form.email)
}
