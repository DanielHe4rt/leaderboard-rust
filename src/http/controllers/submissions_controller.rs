use actix_web::{get, HttpResponse, post, Responder, Result, web};
use charybdis::operations::{Find, Insert};
use charybdis::types::Uuid;
use serde_json::json;
use validator::Validate;

use crate::AppState;
use crate::http::FuckThatError;
use crate::http::requests::submission::{SubmissionDTO, SubmissionResponse};
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
        .await;

    let response = match submission {
        Ok(submission) => HttpResponse::Ok().json(json!(SubmissionResponse::found(submission.clone()))),
        Err(_) => HttpResponse::NotFound().json(json!(SubmissionResponse::not_found())),
    };

    Ok(response)
}

#[post("/submissions")]
async fn post_submission(
    data: web::Data<AppState>,
    payload: web::Json<SubmissionDTO>,
) -> Result<impl Responder, FuckThatError> {
    let validated = payload.validate();

    let response = match validated {
        Ok(_) => {
            Leaderboard::from_request(&payload)
                .await
                .insert(&data.database)
                .await?;

            let submission = Submission::from_request(&payload).await;
            let _ = submission.insert(&data.database).await;
            HttpResponse::Ok().json(json!(SubmissionResponse::created(submission)))
        }
        Err(err) => HttpResponse::BadRequest().json(json!(err)),
    };

    Ok(response)
}