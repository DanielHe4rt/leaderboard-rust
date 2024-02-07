use actix_web::{HttpResponse, ResponseError};
use charybdis::errors::CharybdisError;
use serde_json::json;
use thiserror::Error;

pub mod controllers;
pub mod requests;


#[derive(Error, Debug)]
pub enum FuckThatError {
    #[error("Charybdis error: {0}")]
    CharybdisError(#[from] CharybdisError),
}

impl ResponseError for FuckThatError {
    fn error_response(&self) -> HttpResponse {
        match self {
            FuckThatError::CharybdisError(e) => match e {
                CharybdisError::NotFoundError(e) => HttpResponse::NotFound().json(json!({
                    "status": 404,
                    "message": e.to_string()
                })),
                _ => HttpResponse::InternalServerError().json(json!({
                    "status": 500,
                    "message": e.to_string()
                }))
            }
        }
    }
}