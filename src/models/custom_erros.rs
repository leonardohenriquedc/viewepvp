use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub enum CustomError {
    PermissionDenied(String),
    ThisObjectAlreadyExists,
    ErrorCreating,
    NotFound(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CustomError::PermissionDenied(ref user) => {
                write!(f, "User {} does not have permission", user)
            }
            CustomError::ThisObjectAlreadyExists => write!(f, "This object already exists"),
            CustomError::ErrorCreating => write!(f, "Error creating object"),
            CustomError::NotFound(ref obj) => {
                write!(f, "This object: {} not found", obj)
            }
        }
    }
}
