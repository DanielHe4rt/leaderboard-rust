use std::sync::Arc;
use actix_web::web::Path;

use charybdis::operations::{Find, Insert};
use charybdis::types::Uuid;
use scylla::CachingSession;

use crate::http::SomeError;
use crate::http::requests::submission_request::SubmissionDTO;
use crate::models::submission::Submission;

pub struct SubmissionRepository {
    db: Arc<CachingSession>,
}


impl SubmissionRepository {
    pub fn new(db: Arc<CachingSession>) -> Self {
        SubmissionRepository {
            db
        }
    }

    pub async fn insert(&self, payload: &SubmissionDTO) -> actix_web::Result<Submission, SomeError> {
        let submission = Submission::from_request(&payload);
        submission.insert(&self.db).await?;

        Ok(submission)
    }

    pub async fn find_by_id(&self, id: Uuid) -> actix_web::Result<Submission, SomeError> {
        let submission = Submission { id, ..Default::default() };
        let submission = submission.find_by_partition_key(&self.db).await?;
        let submission = submission.try_collect().await?.pop().unwrap();

        Ok(submission)
    }
}