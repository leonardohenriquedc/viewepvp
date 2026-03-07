use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct GroupDto {
    pub name: String,
}
