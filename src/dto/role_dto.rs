use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct RoleDto {
    pub name: String,
}
