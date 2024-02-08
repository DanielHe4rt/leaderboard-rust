use actix_web::{get, HttpResponse, post, Responder, Result, web};
use charybdis::operations::{Find, Insert};
use charybdis::types::Uuid;
use serde_json::json;
use validator::Validate;

use crate::AppState;
use crate::http::FuckThatError;
use crate::http::requests::submission_request::{SubmissionDTO, SubmissionResponse};
use crate::models::leaderboard::Leaderboard;
use crate::models::submission::Submission;


#[get("/submissions/{id}")]
async fn get_submission(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, FuckThatError> {
    let submission = Submission { id: id.into_inner(), ..Default::default() };
    let submission = submission
        .find_by_primary_key(&data.database)
        .await?;

    Ok(HttpResponse::Ok().json(json!(SubmissionResponse::found(submission))))
}

#[post("/submissions")]
async fn post_submission(
    data: web::Data<AppState>,
    payload: web::Json<SubmissionDTO>,
) -> Result<impl Responder, FuckThatError> {
    let validated = payload.validate();

    let response = match validated {
        Ok(_) => {

            let leaderboard = Leaderboard::from_request(&payload);
            leaderboard.insert(&data.database).await?;

            let submission = Submission::from_request(&payload);
            submission.insert(&data.database).await?;

            HttpResponse::Ok().json(json!(SubmissionResponse::created(submission)))
        }
        Err(err) => HttpResponse::BadRequest().json(json!(err)),
    };

    Ok(response)
}