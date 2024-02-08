use charybdis::types::{Frozen, Int, Set, Text, Timestamp, Uuid};
use charybdis_macros::charybdis_model;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::http::requests::submission::SubmissionDTO;

#[charybdis_model(
    table_name = song_leaderboard,
    partition_keys = [song_id, modifiers, difficulty, instrument],
    clustering_keys = [score, player_id],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    table_options = "
      CLUSTERING ORDER BY (score DESC, player_id ASC)
    ",
)]
#[derive(Serialize, Deserialize, Default, Clone, Validate)]
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
            difficulty: payload.player_id.to_string(),
            instrument: payload.instrument.to_string(),
            modifiers: payload.modifiers.to_owned(),
            score: payload.score.to_owned(),
            played_at: chrono::Utc::now(),
            ..Default::default()
        }
    }
}