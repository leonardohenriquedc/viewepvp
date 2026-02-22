use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePlayerRequest {
    pub nickname: String,
    pub real_name: String,
}

#[derive(Serialize)]
pub struct PlayerResponse {
    pub nickname: String,
    pub real_name: String,
}
