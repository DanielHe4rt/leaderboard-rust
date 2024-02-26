use std::sync::Arc;

use charybdis::operations::{execute, Find, Insert};
use charybdis::stream::CharybdisModelStream;
use charybdis::types::{Set, Text};
use scylla::CachingSession;

use crate::http::requests::leaderboard_request::LeaderboardRequest;
use crate::http::requests::submission_request::SubmissionDTO;
use crate::http::SomeError;
use crate::models::leaderboard::Leaderboard;

pub struct LeaderboardRepository {
    db: Arc<CachingSession>,
}


impl LeaderboardRepository {
    pub fn new(db: Arc<CachingSession>) -> Self {
        LeaderboardRepository {
            db
        }
    }

    pub async fn insert(&self, payload: &SubmissionDTO) -> actix_web::Result<(), SomeError> {
        let leaderboard = Leaderboard::from_request(&payload);
        leaderboard.insert().execute(&self.db).await?;

        Ok(())
    }

    pub async fn update_score(&self, payload: &SubmissionDTO) -> actix_web::Result<(), SomeError> {
        let query = "DELETE FROM leaderboard.song_leaderboard WHERE song_id = ? AND modifiers = ? AND difficulty = ? AND instrument = ? AND player_id = ? AND score < ?";

        let response = execute(&self.db, query, (
            payload.song_id.to_owned(),
            payload.modifiers.to_owned(),
            payload.difficulty.to_owned(),
            payload.instrument.to_owned(),
            payload.player_id.to_owned(),
            payload.score.to_owned(),
        )).await?;

        println!("{:?}", response);

        Ok(())
    }

    pub async fn get_leaderboard(&self, song_id: Text, payload: LeaderboardRequest) -> actix_web::Result<CharybdisModelStream<Leaderboard>, SomeError> {
        let mut modifiers = Set::new();
        modifiers.insert(String::from("no-modifiers"));

        let leaderboard = Leaderboard {
            song_id: song_id.to_string(),
            instrument: payload.instrument.to_owned(),
            difficulty: payload.difficulty.to_owned(),
            modifiers: modifiers.to_owned(),
            ..Default::default()
        };

        let leaderboard = leaderboard.find_by_partition_key().execute(&self.db).await?;

        Ok(leaderboard)
    }
}