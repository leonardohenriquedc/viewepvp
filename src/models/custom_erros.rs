use core::fmt;
use std::fmt::write;

pub enum CreatingError {
    PermissionDenied(String),
    ThisObjectAlreadyExists,
}

impl fmt::Display for CreatingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CreatingError::PermissionDenied(ref User) => {
                write!(f, "User {} does not have permission", User)
            }
            CreatingError::ThisObjectAlreadyExists => write!(f, "This object already exists"),
        }
    }
}
