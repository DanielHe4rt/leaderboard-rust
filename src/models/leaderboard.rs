use charybdis::macros::charybdis_model;
use charybdis::types::{Frozen, Int, Set, Text, Timestamp, Uuid};
use serde::{Deserialize, Serialize};

use crate::http::requests::submission_request::SubmissionDTO;

#[charybdis_model(
table_name = song_leaderboard,
partition_keys = [song_id, modifiers, difficulty, instrument],
clustering_keys = [player_id, score],
global_secondary_indexes = [],
local_secondary_indexes = [],
table_options = "
  CLUSTERING ORDER BY (player_id ASC, score DESC)
",
)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Leaderboard {
    pub id: Uuid,
    pub song_id: Text,
    pub player_id: Text,
    pub modifiers: Frozen<Set<Text>>,
    pub score: Int,
    pub difficulty: Text,
    pub instrument: Text,
    pub played_at: Timestamp,
}

impl Leaderboard {
    pub fn from_request(payload: &SubmissionDTO) -> Self {
        Leaderboard {
            id: Uuid::new_v4(),
            song_id: payload.song_id.to_string(),
            player_id: payload.player_id.to_string(),
            difficulty: payload.difficulty.to_string(),
            instrument: payload.instrument.to_string(),
            modifiers: payload.modifiers.to_owned(),
            score: payload.score.to_owned(),
            played_at: chrono::Utc::now(),
            ..Default::default()
        }
    }
}