use actix_web::{HttpResponse, post, Responder, Result, web};
use charybdis::operations::Insert;
use serde_json::json;
use validator::Validate;

use crate::AppState;
use crate::http::FuckThatError;
use crate::http::requests::submission::{SubmissionDTO, SubmissionResponse};
use crate::models::leaderboard::Leaderboard;
use crate::models::submission::Submission;

#[post("/submissions")]
async fn post_submission(
    data: web::Data<AppState>,
    payload: web::Json<SubmissionDTO>,
) -> Result<impl Responder, FuckThatError> {
    let validated = payload.validate();

    Leaderboard::from_request(&payload)
        .await
        .insert(&data.database)
        .await?;

    let submission =  Submission::from_request(&payload).await;

    let response = match validated {
        Ok(_) => HttpResponse::Ok().json(json!(SubmissionResponse::success(submission))),
        Err(err) => HttpResponse::BadRequest().json(json!(err)),
    };

    Ok(response)
}