use std::fmt::Debug;

use thiserror::Error;

pub use crate::exception;
pub use crate::success;

pub use self::Exception::*;

#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum Exception {
    #[error("{0}")]
    Exception(String),
    #[error("{0}")]
    TypeError(String),
    #[error("Cannot find variable \"{0}\"")]
    ReferenceError(String),
}
