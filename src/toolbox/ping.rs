use actix_web::{HttpResponse, http};


pub async fn ping() -> HttpResponse {
    let http_response = HttpResponse::Ok().body("pong!".to_string());
    println!("This is a simple http response: {:?}", http_response);
    http_response
}

#[cfg(test)]
mod test {
    use super::*;
    // use actix_web::test;

    #[actix_rt::test]
    async fn ping_is_ok() {
        let ping_response = ping().await;
        assert_eq!(ping_response.status(), http::StatusCode::OK);
    }

}