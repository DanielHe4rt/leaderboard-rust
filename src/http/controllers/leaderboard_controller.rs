use actix_web::{get, HttpResponse, Responder, web};
use charybdis::types::Text;
use serde_json::json;

use crate::config::app::AppState;
use crate::http::requests::leaderboard_request::LeaderboardRequest;
use crate::http::SomeError;
use crate::repositories::leaderboard_repository::LeaderboardRepository;

#[get("/leaderboard/{song_id}")]
async fn get_leaderboard(
    data: web::Data<AppState>,
    song_id: web::Path<Text>,
    payload: web::Query<LeaderboardRequest>,
) -> actix_web::Result<impl Responder, SomeError> {
    let leaderboard_repository = LeaderboardRepository::new(data.database.clone());

    let song_leaderboard = leaderboard_repository.get_leaderboard(
        song_id.to_owned(),
        payload.into_inner()
    ).await?;

    Ok(HttpResponse::Ok().json(json!(song_leaderboard.try_collect().await?)))
}