use actix_web::{get, HttpResponse, post, Responder, Result, web};
use charybdis::operations::Insert;
use charybdis::types::Uuid;
use serde_json::json;
use validator::Validate;

use crate::AppState;
use crate::http::requests::submission_request::{SubmissionDTO, SubmissionResponse};
use crate::http::SomeError;
use crate::repositories::leaderboard_repository::LeaderboardRepository;
use crate::repositories::submission_repository::SubmissionRepository;

#[get("/submissions/{id}")]
async fn get_submission(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, SomeError> {
    let submission_repository = SubmissionRepository::new(data.database.clone());
    let submission = submission_repository.find_by_id(id.to_owned()).await?;

    Ok(HttpResponse::Ok().json(json!(SubmissionResponse::found(submission))))
}

#[post("/submissions")]
async fn post_submission(
    data: web::Data<AppState>,
    payload: web::Json<SubmissionDTO>,
) -> Result<impl Responder, SomeError> {
    let validated = payload.validate();
    let leaderboard_repository = LeaderboardRepository::new(data.database.clone());
    let submission_repository = SubmissionRepository::new(data.database.clone());

    let response = match validated {
        Ok(_) => {
            let submission = submission_repository.insert(&payload).await?;
            leaderboard_repository.insert(&payload).await?;
            leaderboard_repository.update_score(&payload).await?;

            HttpResponse::Ok().json(json!(SubmissionResponse::created(submission)))
        }
        Err(err) => HttpResponse::BadRequest().json(json!(err)),
    };

    Ok(response)
}