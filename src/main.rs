use actix_web::{App, HttpServer};
use actix_web::web::Data;

use crate::config::app::AppState;

mod models;
mod http;

mod config;

mod repositories;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = AppState::new().await;

    println!("Web Server Online!");
    println!("Listening on http://{}:{}", app_data.config.app.url, app_data.config.app.port);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState::new()))
    }).bind((
        app_data.config.app.url,
        app_data.config.app.port.parse::<u16>().unwrap()
    ))?.run().await
}