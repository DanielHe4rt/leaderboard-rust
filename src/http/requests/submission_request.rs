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
    pub submission: Option<Submission>,
}

impl SubmissionResponse {
    pub fn created(submission: Submission) -> Self {
        SubmissionResponse {
            message: "Submission created successfully".to_string(),
            submission: Some(submission)
        }
    }

    pub fn found(submission: Submission) -> Self {
        SubmissionResponse {
            message: "Submission found.".to_string(),
            submission: Some(submission)
        }
    }

    pub fn not_found() -> Self {
        SubmissionResponse {
            message: "Submission not found.".to_string(),
            submission: None
        }
    }
}
