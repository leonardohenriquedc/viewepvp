use core::fmt;
use std::fmt::write;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub enum CustomError {
    PermissionDenied(String),
    ThisObjectAlreadyExists,
    ErrorCreating,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CustomError::PermissionDenied(ref User) => {
                write!(f, "User {} does not have permission", User)
            }
            CustomError::ThisObjectAlreadyExists => write!(f, "This object already exists"),
            CustomError::ErrorCreating => write!(f, "Error creating object"),
        }
    }
}
