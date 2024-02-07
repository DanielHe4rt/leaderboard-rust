use charybdis::types::{Frozen, Int, Set, Text};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::submission::Submission;

#[derive(Deserialize, Debug, Validate)]
pub struct SubmissionDTO {
    pub song_id: Text,
    pub player_id: Text,
    pub modifiers: Frozen<Set<Text>>,
    pub score: Int,
    pub difficulty: Text,
    pub instrument: Text,
}


#[derive(Serialize)]
pub struct SubmissionResponse {
    pub message: String,
    pub submission: Submission,
}

impl SubmissionResponse {
    pub fn success(submission: Submission) -> Self {
        SubmissionResponse {
            message: "Submission created successfully".to_string(),
            submission
        }
    }
}
