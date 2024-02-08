use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Config {
    pub app_name: String,
    pub app_version: String,
}