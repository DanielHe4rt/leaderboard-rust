use charybdis::types::Text;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LeaderboardRequest {
    pub instrument: Text,
    pub difficulty: Text,
    //pub modifiers: Frozen<Set<Text>>,
}