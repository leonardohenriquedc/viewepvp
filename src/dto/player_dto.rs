use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePlayerRequest {
    pub nickname: String,
    pub real_name: String,
}
