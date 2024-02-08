use actix_web::{App, HttpServer};
use actix_web::web::Data;

use crate::app::AppState;
use crate::http::controllers::submissions_controller::{get_submission, post_submission};

mod models;
mod http;
mod config;
mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let app_data = AppState::new().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_data.clone()))
            .service(post_submission)
            .service(get_submission)
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
