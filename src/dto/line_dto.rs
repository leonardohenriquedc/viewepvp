use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateLineRequest {
    pub name: i32,
    pub nicknames: [String; 5],
}

#[derive(Serialize)]
pub struct LineWithPlayersResponse {
    pub id: i32,
    pub name: i32,
    pub nicknames: Vec<String>,
}
