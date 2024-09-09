use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::error::Error;
use std::fmt::Display;
use api_result::{ApiError, ApiResult};

mod display;
mod convert;

/// The result type of this crate.
pub type YxResult<T> = ApiResult<T, YxError>;

/// A boxed error kind, wrapping an [YxErrorKind].
#[derive(Clone)]
pub struct YxError {
    kind: Box<YxErrorKind>,
}

/// The kind of [YxError].
#[derive(Debug, Clone)]
pub enum YxErrorKind {
    /// An unknown error.
    UnknownError,

    DatabaseError {
        message: String,
    },
    ServiceError {
        message: String,
    },
}

impl ApiError for YxError {
    fn error_code(&self) -> i32 {
        match self.kind.as_ref() {
            YxErrorKind::UnknownError => { -1 }
            YxErrorKind::DatabaseError { .. } => {
                -100
            }
            YxErrorKind::ServiceError { .. } => {
                -200
            }
        }
    }
    fn error_message(&self) -> Cow<str> {
        Cow::Owned(self.to_string())
    }
}

impl YxError {
    pub fn database_error(message: impl Into<String>) -> YxError {
        YxError {
            kind: Box::new(YxErrorKind::DatabaseError {
                message: message.into(),
            }),
        }
    }
    pub fn service_error(message: impl Into<String>) -> YxError {
        YxError {
            kind: Box::new(YxErrorKind::ServiceError {
                message: message.into(),
            }),
        }
    }

}
