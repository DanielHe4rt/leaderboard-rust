use std::sync::Arc;
use std::time::Duration;

use actix_web::{App, HttpServer};
use actix_web::web::Data;
use dotenvy::dotenv;
use scylla::{CachingSession, Session, SessionBuilder};
use serde::Serialize;

use crate::http::controllers::submissions_controller::{get_submission, post_submission};

mod models;
mod http;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");

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

#[derive(Clone, Debug, Serialize)]
struct Config {
    pub app_name: String,
    pub app_version: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    config: Config,
    pub database: Arc<CachingSession>
}

impl AppState {
    pub async fn new() -> Self {
        let nodes: Vec<String> = dotenvy::var("SCYLLA_NODES").unwrap().split(',').map(|s| s.to_string()).collect();
        let username = dotenvy::var("SCYLLA_USERNAME").unwrap();
        let password = dotenvy::var("SCYLLA_PASSWORD").unwrap();
        let cached_queries_count = dotenvy::var("SCYLLA_CACHED_QUERIES").unwrap().parse::<usize>().unwrap();


        let session: Session = SessionBuilder::new()
            .known_nodes(nodes)
            .connection_timeout(Duration::from_secs(5))
            .user(username.to_string(), password.to_string())
            .build()
            .await
            .expect("Connection Refused. Check your credentials and your IP linked on the ScyllaDB Cloud.");

        session.use_keyspace("leaderboard", false).await.expect("Keyspace not found");

        AppState {
            config: Config {
                app_name: dotenvy::var("APP_NAME").unwrap(),
                app_version: dotenvy::var("APP_VERSION").unwrap()
            },
            database: Arc::new(CachingSession::from(session, cached_queries_count))
        }
    }
}