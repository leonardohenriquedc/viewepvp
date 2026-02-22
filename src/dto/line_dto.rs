use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateLineRequest {
    pub name: String,
    pub nicknames: [String; 5],
}

#[derive(Serialize)]
pub struct LineWithPlayersResponse {
    pub name: String,
    pub nicknames: Vec<String>,
}
