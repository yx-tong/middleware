use poem_result::{ApiError, PoemResult};
use std::{
    borrow::Cow,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

mod convert;
mod display;
mod for_lettre;
#[cfg(feature = "pdf-extract")]
mod for_pdf_extract;
mod for_sqlx;

/// The result type of this crate.
pub type YxResult<T> = PoemResult<T, YxError>;

/// A boxed error kind, wrapping an [YxErrorKind].
#[derive(Clone)]
pub struct YxError {
    kind: Box<YxErrorKind>,
}

/// The kind of [YxError].
#[derive(Debug, Clone)]
pub enum YxErrorKind {
    DatabaseError {
        message: String,
    },
    ServiceError {
        message: String,
    },
    EncodeError {
        format: String,
        message: String,
    },
    DecodeError {
        format: String,
        message: String,
    },
    /// An unknown error.
    UnknownError,
}

impl ApiError for YxError {
    fn error_code(&self) -> i32 {
        match self.kind.as_ref() {
            YxErrorKind::UnknownError => -1,
            YxErrorKind::DatabaseError { .. } => -100,
            YxErrorKind::ServiceError { .. } => -200,
            YxErrorKind::EncodeError { .. } => -400,
            YxErrorKind::DecodeError { .. } => -400,
        }
    }
    fn error_message(&self) -> Cow<str> {
        Cow::Owned(self.to_string())
    }
}

impl YxError {
    pub fn database_error(message: impl Into<String>) -> YxError {
        YxError { kind: Box::new(YxErrorKind::DatabaseError { message: message.into() }) }
    }
    pub fn service_error(message: impl Into<String>) -> YxError {
        YxError { kind: Box::new(YxErrorKind::ServiceError { message: message.into() }) }
    }
    pub fn encode_error(message: impl Into<String>) -> YxError {
        YxError { kind: Box::new(YxErrorKind::EncodeError { format: "".to_string(), message: message.into() }) }
    }
    pub fn decode_error(message: impl Into<String>) -> YxError {
        YxError { kind: Box::new(YxErrorKind::DecodeError { format: "".to_string(), message: message.into() }) }
    }
}
