use actix_web::{HttpResponse};

pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong!".to_string())
}
