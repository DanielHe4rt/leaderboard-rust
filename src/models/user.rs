use charybdis::types::{Text, Timestamp, Uuid};
use charybdis_macros::charybdis_model;

#[charybdis_model(
table_name = users,
partition_keys = [id],
clustering_keys = [],
global_secondary_indexes = [],
local_secondary_indexes = [],
)]
pub struct User {
    pub id: Uuid,
    pub username: Text,
    pub email: Text,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub address: Text,
}