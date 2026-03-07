use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug, Clone)]
pub struct UserDto {
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}
