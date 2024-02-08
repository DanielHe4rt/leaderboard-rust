use std::sync::Arc;
use std::time::Duration;
use dotenvy::dotenv;
use scylla::{CachingSession, Session, SessionBuilder};
use crate::config::Config;


#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub database: Arc<CachingSession>
}

impl AppState {
    pub async fn new() -> Self {
        dotenv().expect(".env file not found");
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