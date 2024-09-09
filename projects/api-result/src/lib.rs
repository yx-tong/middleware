#![feature(try_trait_v2)]

#[cfg(feature = "axum")]
mod for_axum;
mod for_serde;
mod for_error;
mod for_poem;


#[cfg(feature = "utoipa")]
mod for_utoipa;


use std::borrow::Cow;
pub use ApiResult::{Success, Failure};

#[derive(Debug, Clone, PartialEq)]
pub enum ApiResult<T, E> {
    Success(T),
    Failure(E),
}

pub trait ApiError {
    fn error_code(&self) -> i32 {
        -1
    }
    fn error_message(&self) -> Cow<str> {
        Cow::Borrowed("")
    }
}
