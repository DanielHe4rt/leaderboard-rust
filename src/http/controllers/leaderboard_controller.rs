use actix_web::{get, HttpResponse, Responder, web};
use charybdis::operations::Find;
use charybdis::types::{Set, Text};
use serde_json::json;

use crate::app::AppState;
use crate::http::FuckThatError;
use crate::http::requests::leaderboard_request::LeaderboardRequest;
use crate::models::leaderboard::Leaderboard;

#[get("/leaderboard/{song_id}")]
async fn get_leaderboard(
    data: web::Data<AppState>,
    song_id: web::Path<Text>,
    payload: web::Query<LeaderboardRequest>
) -> actix_web::Result<impl Responder, FuckThatError> {
    let mut modifiers = Set::new();
    modifiers.insert(String::from("no-modifiers"));

    let leaderboard = Leaderboard {
        song_id: song_id.to_string(),
        instrument: payload.instrument.to_owned(),
        difficulty: payload.difficulty.to_owned(),
        modifiers: modifiers.to_owned(),
        ..Default::default()
    };

    let song_leaderboard = leaderboard.find_by_partition_key(&data.database).await?;

    Ok(HttpResponse::Ok().json(json!(song_leaderboard.try_collect().await?)))
}