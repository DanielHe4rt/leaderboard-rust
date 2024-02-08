use charybdis::types::{Frozen, Set, Text};
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct LeaderboardRequest {
    pub instrument: Text,
    pub difficulty: Text,
    //pub modifiers: Frozen<Set<Text>>,
}