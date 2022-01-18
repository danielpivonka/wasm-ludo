use actix_web::{get, HttpResponse};

#[get("")]
pub async fn get_info() -> HttpResponse {
  HttpResponse::Ok().body("hello from the server")
}
