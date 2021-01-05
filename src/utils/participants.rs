use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Participants {
    #[serde(rename = "userFrom")]
    pub user_from: String,
    #[serde(rename = "userTo")]
    pub user_to: String,
}
