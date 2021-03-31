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

#[macro_export]
macro_rules! success {
    ($value:expr) => { Ok($value)};
    () => {Ok(crate::runtime::Value::Undefined)};
}

#[macro_export]
macro_rules! exception {
    ($value:expr) => {std::result::Result::Err($value)};
}
