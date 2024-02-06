use actix_web::{get, HttpResponse, post, Responder, Result, web};
use serde::Serialize;
use crate::AppState;


#[derive(Serialize, Debug)]
pub struct PingResponse {
    pub message: String,
    pub version: String,
}

#[get("/echo")]
async fn echo(data: web::Data<AppState>) -> Result<impl Responder> {
    let response = PingResponse {
        message: "Hello, world!".to_string(),
        version: data.config.app_version.clone()
    };

    Ok(HttpResponse::Ok().json(response))
}